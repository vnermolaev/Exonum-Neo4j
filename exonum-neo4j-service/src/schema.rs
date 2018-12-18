// Copyright 2018 The Exonum Team
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//   http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! Cryptocurrency database schema.

use exonum::{
    crypto::{hash, Hash}, storage::{Fork, ProofListIndex, ProofMapIndex, Snapshot,},
};

use structures::{Queries, NodeChange, Relation};


/// Database schema for the cryptocurrency.
#[derive(Debug)]
pub struct Schema<T> {
    view: T,
}

impl<T> AsMut<T> for Schema<T> {
    fn as_mut(&mut self) -> &mut T {
        &mut self.view
    }
}

impl<T> Schema<T>
where
    T: AsRef<dyn Snapshot>,
{
    /// Creates a new schema from the database view.
    pub fn new(view: T) -> Self {
        Schema { view }
    }

    /// Returns `ProofMapIndex` with queries.
    pub fn queries(&self) -> ProofMapIndex<&T, Hash, Queries> {
        ProofMapIndex::new("neo4j.queries", &self.view)
    }

    ///Get a single query, by giving transaction hash as key
    pub fn query(&self, q_hash: &Hash) -> Option<Queries> {
        self.queries().get(q_hash)
    }

    ///Get relations ProofMapIndex
    pub fn relations(&self) -> ProofMapIndex<&T, Hash, Relation> {
        ProofMapIndex::new("neo4j.relations", &self.view)
    }

    ///Get a single relation, by giving it's uuid as key
    pub fn relation(&self, relation_uuid: &str) -> Option<Relation> {
        self.relations().get(&hash(relation_uuid.as_bytes()))
    }

    ///Get a node's history proofListIndex by giving that node's uuid.
    pub fn node_history(&self, node_name: &str) -> ProofListIndex<&T, NodeChange> {
        ProofListIndex::new(format!("neo4j.node_changes_{}", node_name), &self.view)

    }

    ///Get state hash
    pub fn state_hash(&self) -> Vec<Hash> {
        vec![self.queries().merkle_root()]
    }
}

/// Implementation of mutable methods.
impl<'a> Schema<&'a mut Fork> {
    ///Get all queries from database.
    pub fn queries_mut(&mut self) -> ProofMapIndex<&mut Fork, Hash, Queries> {
        ProofMapIndex::new("neo4j.queries", &mut self.view)
    }

    ///Add a new variable to the table.
    pub fn add_query(&mut self, q: Queries) {
        let hash = q.transaction_hash().clone();
        self.queries_mut().put(&hash, q);

    }

    ///Get mutable relations proofmapindex
    pub fn relations_mut(&mut self) -> ProofMapIndex<&mut Fork, Hash, Relation> {
        ProofMapIndex::new("neo4j.relations", &mut self.view)
    }

    ///Add a new variable to the table.
    pub fn add_relation(&mut self, r: Relation, relation_uuid : &str) {
        let hash = hash(relation_uuid.as_bytes());
        self.relations_mut().put(&hash, r);

    }

    ///Get a mutable prooflistindex for a node's history
    pub fn node_history_mut(&mut self, node_name: &str) -> ProofListIndex<&mut Fork, NodeChange> {
        ProofListIndex::new(format!("neo4j.node_changes_{}", node_name), &mut self.view)
    }

    ///Add to node history
    pub fn add_node_history(&mut self, uuid: &str, node_change: &NodeChange){
        self.node_history_mut(uuid).push(node_change.clone())
    }
}

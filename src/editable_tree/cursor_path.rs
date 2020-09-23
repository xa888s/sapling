use crate::vec_node_map::Index;

/// One part of a path from the root of a tree to the cursor.
/// A [`Vec`] of these allows the [`DAG`] [`EditableTree`] to climb back up the trees to the root
/// without having to keep backpointers updated in the DAG.
/// Keeping backpointers inside a **DAG** is particularly probablematic since each node can (and
/// often will) have multiple parents and therefore it's very badly defined which one to use.
#[derive(Debug, Clone)]
pub(super) struct Segment {
    pub node_index: Index,
    pub sibling_index: usize,
}

impl Segment {
    /// Constructs a new `CursorLocationSegment` from its component parts
    pub fn new(node_index: Index, sibling_index: usize) -> Self {
        Segment {
            node_index,
            sibling_index,
        }
    }

    /// Constructs a `CursorLocationSegment` that is correct for representing the root of a tree
    pub fn root(node_index: Index) -> Self {
        Self::new(node_index, 0)
    }
}
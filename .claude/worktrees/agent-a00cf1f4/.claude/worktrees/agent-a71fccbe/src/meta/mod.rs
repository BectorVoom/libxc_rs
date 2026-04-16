pub(crate) mod generated;

use crate::model::{
    DerivativeOrder, Family, FunctionalFlags, FunctionalId, HybridTermKind, Kind,
};

/// Literature reference
#[derive(Debug, Clone, Copy)]
pub struct Reference {
    pub citation: &'static str,
    pub doi: &'static str,
    pub bibtex: &'static str,
    pub key: &'static str,
}

/// External parameter specification
#[derive(Debug, Clone, Copy)]
pub struct ExtParamSpec {
    pub name: &'static str,
    pub description: &'static str,
    pub default_value: f64,
    /// If true, this is an internal parameter (name starts with '_')
    pub is_internal: bool,
}

/// A single hybrid exchange term
#[derive(Debug, Clone, Copy)]
pub struct HybridTerm {
    pub kind: HybridTermKind,
    pub coefficient: f64,
    pub omega: f64,
}

/// Static metadata for a functional. Lives in .rodata.
#[derive(Debug)]
pub struct FunctionalMeta {
    pub id: FunctionalId,
    pub name: &'static str,
    pub kind: Kind,
    pub family: Family,
    pub flags: FunctionalFlags,
    pub references: &'static [Reference],
    pub ext_params: &'static [ExtParamSpec],
    pub default_density_threshold: f64,
    /// Auxiliary functional IDs and weights for mixed/hybrid functionals
    pub auxiliaries: &'static [(FunctionalId, f64)],
    /// Hybrid term definitions
    pub hybrid_terms: &'static [HybridTerm],
    /// Non-local correlation parameters (b, C) if applicable
    pub nlc_params: Option<(f64, f64)>,
    /// Maximum supported derivative order
    pub max_order: DerivativeOrder,
}

//! LDA functionals deferred from translation due to CubeCL proc macro stack limits.
//!
//! These 4 functionals were translated with the incremental derivative structure
//! (so source files exist under their respective modules), but the generated
//! `kxc_pol` or `lxc_pol` functions exceed CubeCL's proc-macro stack limit
//! (~10K lines per `#[cube]` fn). Compiling them aborts with stack overflow.
//!
//! To enable these functionals, split the offending functions per-output-field
//! (as was done for crates/kernel-gga-1a/src/gga_c_ft97/lxc_pol_part{N}.rs)
//! and re-run translate_lda_v2.py.

/// A deferred LDA functional that cannot currently be compiled.
pub struct DeferredLda {
    /// Functional name (e.g., "lda_c_pk09")
    pub name: &'static str,
    /// libxc functional ID
    pub id: u16,
    /// C source file line count of the offending derivative function
    pub c_lines: u32,
    /// Which generated function blocks compilation
    pub blocked_by: &'static str,
    /// Human-readable reason for deferral
    pub reason: &'static str,
}

/// The 4 LDA functionals deferred due to CubeCL proc macro stack limits.
///
/// Total LDA functionals: 41 (37 compiled + 4 deferred)
pub const DEFERRED_LDA_FUNCTIONALS: &[DeferredLda] = &[
    DeferredLda {
        name: "lda_c_pk09",
        id: 554,
        c_lines: 17_500,
        blocked_by: "kxc_pol (17.5K lines exceeds CubeCL proc-macro stack limit)",
        reason: "kxc_pol single function too large to compile via CubeCL proc macro",
    },
    DeferredLda {
        name: "lda_xc_ksdt",
        id: 259,
        c_lines: 14_000,
        blocked_by: "lxc_pol (14K lines exceeds CubeCL proc-macro stack limit)",
        reason: "lxc_pol single function too large to compile via CubeCL proc macro",
    },
    DeferredLda {
        name: "lda_c_pw_erf",
        id: 654,
        c_lines: 11_000,
        blocked_by: "lxc_pol (11K lines exceeds CubeCL proc-macro stack limit)",
        reason: "lxc_pol single function too large to compile via CubeCL proc macro",
    },
    DeferredLda {
        name: "lda_c_pmgb06",
        id: 590,
        c_lines: 9_800,
        blocked_by: "lxc_pol (9.8K lines exceeds CubeCL proc-macro stack limit)",
        reason: "lxc_pol single function too large to compile via CubeCL proc macro",
    },
];

/// Number of LDA functionals that were successfully compiled.
pub const TRANSLATED_LDA_COUNT: usize = 37;

/// Total LDA functionals in libxc 7.0.0 that were translated.
pub const TOTAL_LDA_COUNT: usize = 41;

/// Return true if the given libxc functional ID is in the deferred list.
pub fn is_deferred(id: u16) -> bool {
    DEFERRED_LDA_FUNCTIONALS.iter().any(|d| d.id == id)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deferred_count_matches_constant() {
        assert_eq!(DEFERRED_LDA_FUNCTIONALS.len(), 4);
        assert_eq!(TOTAL_LDA_COUNT - TRANSLATED_LDA_COUNT, 4);
    }

    #[test]
    fn is_deferred_recognizes_known_ids() {
        assert!(is_deferred(554)); // lda_c_pk09
        assert!(is_deferred(259)); // lda_xc_ksdt
        assert!(is_deferred(654)); // lda_c_pw_erf
        assert!(is_deferred(590)); // lda_c_pmgb06
    }

    #[test]
    fn is_deferred_rejects_compiled_ids() {
        assert!(!is_deferred(1));   // lda_x
        assert!(!is_deferred(7));   // lda_c_vwn
        assert!(!is_deferred(599)); // lda_xc_tih
    }
}

//! Registry of XC functionals deferred from translation.
//!
//! Relocated here from the per-family façade crates (`libxc_kernel_lda`,
//! `libxc_kernel_mgga`) which were deleted by Phase 11 D-10a's clean-slate
//! restructure. Every per-functional kernel subcrate already depends on
//! `libxc-kernel-math`, and the root crate's model layer depends on it too,
//! so this is the natural home for the cross-cutting deferral registry.
//!
//! The registry drives runtime routing rejection: `LdaFunctional::from_id`
//! and `MggaFunctional::from_id` consult `lda::is_deferred` / `mgga::is_deferred`
//! to surface a typed `UnsupportedFunctional` error instead of attempting to
//! launch a kernel that cannot currently be compiled.

/// LDA functionals deferred from translation due to CubeCL proc macro stack limits.
pub mod lda {
    //! These 4 functionals were translated with the incremental derivative
    //! structure (so source files exist under their respective modules), but
    //! the generated `kxc_pol` or `lxc_pol` functions exceed CubeCL's
    //! proc-macro stack limit (~10K lines per `#[cube]` fn). Compiling them
    //! aborts with stack overflow.

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
            assert!(!is_deferred(1)); // lda_x
            assert!(!is_deferred(7)); // lda_c_vwn
            assert!(!is_deferred(599)); // lda_xc_tih
        }
    }
}

/// MGGA functionals deferred from translation due to missing math primitives.
pub mod mgga {
    //! These 6 functionals require iterative root-finder implementations
    //! (Brent's method) that are not yet available in kernel-math. They use
    //! either `xc_mgga_x_br89_get_x` (solves x * exp(-2x/3) = C) or
    //! `xc_mgga_x_mbrxc_get_x` (similar nonlinear solve).

    /// A deferred MGGA functional that could not be translated.
    pub struct DeferredMgga {
        /// Functional name (e.g., "mgga_c_b94")
        pub name: &'static str,
        /// libxc functional ID
        pub id: u16,
        /// C source file line count
        pub c_lines: u32,
        /// The special math function this functional requires
        pub blocked_by: &'static str,
        /// Human-readable reason for deferral
        pub reason: &'static str,
    }

    /// The 6 MGGA functionals deferred due to missing iterative root-finders.
    ///
    /// Total MGGA functionals: 92 (90 mgga_exc + 2 mgga_vxc)
    /// Translatable: 86
    /// Deferred: 6 (listed here)
    pub const DEFERRED_MGGA_FUNCTIONALS: &[DeferredMgga] = &[
        DeferredMgga {
            name: "mgga_c_b94",
            id: 397,
            c_lines: 34_899,
            blocked_by: "xc_mgga_x_br89_get_x",
            reason: "Requires Brent's method root-finder for BR89 exchange hole model",
        },
        DeferredMgga {
            name: "mgga_x_br89",
            id: 206,
            c_lines: 69_562,
            blocked_by: "xc_mgga_x_br89_get_x",
            reason: "Requires Brent's method root-finder for BR89 exchange hole model",
        },
        DeferredMgga {
            name: "mgga_x_mbr",
            id: 716,
            c_lines: 36_233,
            blocked_by: "xc_mgga_x_br89_get_x",
            reason: "Requires Brent's method root-finder for BR89 exchange hole model",
        },
        DeferredMgga {
            name: "mgga_x_mbrxc_bg",
            id: 696,
            c_lines: 38_682,
            blocked_by: "xc_mgga_x_mbrxc_get_x",
            reason: "Requires MBRXC variant root-finder for modified BR exchange hole",
        },
        DeferredMgga {
            name: "mgga_x_mbrxh_bg",
            id: 697,
            c_lines: 35_453,
            blocked_by: "xc_mgga_x_br89_get_x",
            reason: "Requires Brent's method root-finder for BR89 exchange hole model",
        },
        DeferredMgga {
            name: "mgga_x_mggac",
            id: 711,
            c_lines: 55_752,
            blocked_by: "xc_mgga_x_mbrxc_get_x",
            reason: "Requires MBRXC variant root-finder for modified BR exchange hole",
        },
    ];

    /// Number of MGGA functionals that were successfully translated.
    pub const TRANSLATED_MGGA_COUNT: usize = 86;

    /// Total MGGA functionals in libxc 7.0.0.
    pub const TOTAL_MGGA_COUNT: usize = 92;

    /// Return true if the given libxc functional ID is in the deferred list.
    pub fn is_deferred(id: u16) -> bool {
        DEFERRED_MGGA_FUNCTIONALS.iter().any(|d| d.id == id)
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn deferred_count_matches_constant() {
            assert_eq!(DEFERRED_MGGA_FUNCTIONALS.len(), 6);
        }

        #[test]
        fn is_deferred_recognizes_known_ids() {
            assert!(is_deferred(397)); // mgga_c_b94
            assert!(is_deferred(206)); // mgga_x_br89
            assert!(is_deferred(716)); // mgga_x_mbr
            assert!(is_deferred(696)); // mgga_x_mbrxc_bg
            assert!(is_deferred(697)); // mgga_x_mbrxh_bg
            assert!(is_deferred(711)); // mgga_x_mggac
        }

        #[test]
        fn is_deferred_rejects_compiled_ids() {
            assert!(!is_deferred(202)); // mgga_x_tpss
            assert!(!is_deferred(208)); // mgga_x_tb09
            assert!(!is_deferred(1)); // lda_x
        }
    }
}

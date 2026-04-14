//! MGGA functionals deferred from translation due to missing math primitives.
//!
//! These 6 functionals require iterative root-finder implementations (Brent's method)
//! that are not yet available in kernel-math. They use either `xc_mgga_x_br89_get_x`
//! (solves x * exp(-2x/3) = C) or `xc_mgga_x_mbrxc_get_x` (similar nonlinear solve).
//!
//! To enable these functionals, implement the root-finders as #[cube] functions in
//! crates/kernel-math/ and then re-run translate_mgga.py after removing them from
//! UNIMPLEMENTED_MATH.

/// A deferred MGGA functional that could not be translated.
pub struct DeferredMgga {
    /// Functional name (e.g., "mgga_c_b94")
    pub name: &'static str,
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
        c_lines: 34_899,
        blocked_by: "xc_mgga_x_br89_get_x",
        reason: "Requires Brent's method root-finder for BR89 exchange hole model",
    },
    DeferredMgga {
        name: "mgga_x_br89",
        c_lines: 69_562,
        blocked_by: "xc_mgga_x_br89_get_x",
        reason: "Requires Brent's method root-finder for BR89 exchange hole model",
    },
    DeferredMgga {
        name: "mgga_x_mbr",
        c_lines: 36_233,
        blocked_by: "xc_mgga_x_br89_get_x",
        reason: "Requires Brent's method root-finder for BR89 exchange hole model",
    },
    DeferredMgga {
        name: "mgga_x_mbrxc_bg",
        c_lines: 38_682,
        blocked_by: "xc_mgga_x_mbrxc_get_x",
        reason: "Requires MBRXC variant root-finder for modified BR exchange hole",
    },
    DeferredMgga {
        name: "mgga_x_mbrxh_bg",
        c_lines: 35_453,
        blocked_by: "xc_mgga_x_br89_get_x",
        reason: "Requires Brent's method root-finder for BR89 exchange hole model",
    },
    DeferredMgga {
        name: "mgga_x_mggac",
        c_lines: 55_752,
        blocked_by: "xc_mgga_x_mbrxc_get_x",
        reason: "Requires MBRXC variant root-finder for modified BR exchange hole",
    },
];

/// Number of MGGA functionals that were successfully translated.
pub const TRANSLATED_MGGA_COUNT: usize = 86;

/// Total MGGA functionals in libxc 7.0.0.
pub const TOTAL_MGGA_COUNT: usize = 92;

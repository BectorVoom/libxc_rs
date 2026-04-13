#![allow(clippy::excessive_precision)]
#![allow(clippy::needless_late_init)]
#![allow(clippy::too_many_arguments)]

//! GGA kernel translations from maple2c.
//!
//! 131 GGA functionals total: 106 compiled across 3 sub-crates, 25 deferred
//! due to CubeCL `#[cube(launch_unchecked)]` proc macro memory requirements
//! exceeding available RAM during compilation.
//!
//! Sub-crate split is required because rustc must hold all expanded macro IR
//! in memory for a single crate. Each sub-crate stays under ~35 modules to
//! fit within ~16GB compilation memory budget.

// Re-export sub-crates containing compiled GGA functionals.
pub use libxc_kernel_gga_1 as batch1;
pub use libxc_kernel_gga_2 as batch2;
pub use libxc_kernel_gga_3 as batch3;

// Deferred functionals -- source files present in src/ but not compiled.
// These contain individual #[cube(launch_unchecked)] functions exceeding 5K lines
// (primarily lxc_pol.rs / kxc_pol.rs -- 3rd/4th order polarized derivatives)
// which cause rustc OOM during CubeCL proc macro expansion.
//
// Recovery path: split each deferred functional's large files into smaller
// per-derivative sub-functions, or await CubeCL improvements to reduce
// proc macro memory footprint.
//
// pub mod gga_c_acgga;       // lxc_pol.rs: 11171 lines
// pub mod gga_c_acggap;      // lxc_pol.rs: 17063 lines
// pub mod gga_c_ft97;        // lxc_pol.rs: 37787 lines
// pub mod gga_c_gapc;        // lxc_pol.rs: 15140 lines
// pub mod gga_c_gaploc;      // lxc_pol.rs: 17391 lines
// pub mod gga_c_hcth_a;      // lxc_pol.rs: 7716 lines
// pub mod gga_c_optc;        // lxc_pol.rs: 19357 lines
// pub mod gga_c_pbe_erf_gws; // lxc_pol.rs: 23663 lines
// pub mod gga_c_pbeloc;      // lxc_pol.rs: 7904 lines
// pub mod gga_c_pw91;        // lxc_pol.rs: 7103 lines
// pub mod gga_c_q2d;         // lxc_pol.rs: 17770 lines
// pub mod gga_c_regtpss;     // lxc_pol.rs: 7420 lines
// pub mod gga_c_revtca;      // lxc_pol.rs: 5518 lines
// pub mod gga_c_sg4;         // lxc_pol.rs: 13568 lines
// pub mod gga_c_sogga11;     // lxc_pol.rs: 9835 lines
// pub mod gga_c_zpbeint;     // lxc_pol.rs: 7030 lines
// pub mod gga_c_zvpbeint;    // lxc_pol.rs: 8290 lines
// pub mod gga_c_zvpbeloc;    // lxc_pol.rs: 8162 lines
// pub mod gga_x_ft97;        // lxc_pol.rs: 5721 lines
// pub mod gga_x_hjs;         // lxc_pol.rs: 11495 lines
// pub mod gga_x_hjs_b88_v2;  // lxc_pol.rs: 13303 lines
// pub mod gga_x_lcgau;       // lxc_pol.rs: 7242 lines
// pub mod gga_x_wpbeh;       // lxc_pol.rs: 25973 lines
// pub mod gga_xc_b97;        // lxc_pol.rs: 5816 lines
// pub mod hyb_gga_xc_wb97;   // lxc_pol.rs: 7379 lines

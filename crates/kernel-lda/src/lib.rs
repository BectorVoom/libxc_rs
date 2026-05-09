#![allow(clippy::excessive_precision)]
#![allow(clippy::needless_late_init)]
#![allow(clippy::too_many_arguments)]

//! LDA kernel implementations (incremental derivative structure).
//! Auto-generated: 41 functionals re-exported from 1 sub-crates.
//! Bin-packed by tools/split_lda_subcrates.py

// `deferred` is in-aggregator metadata (not a kernel module).
pub mod deferred;

// Functional re-exports (paths preserved so that
// `use libxc_kernel_lda::<func>::*;` keeps working).
pub use libxc_kernel_lda_1::hyb_lda_xc_bn05;
pub use libxc_kernel_lda_1::lda_c_1d_csc;
pub use libxc_kernel_lda_1::lda_c_1d_loos;
pub use libxc_kernel_lda_1::lda_c_2d_amgb;
pub use libxc_kernel_lda_1::lda_c_2d_prm;
pub use libxc_kernel_lda_1::lda_c_chachiyo;
pub use libxc_kernel_lda_1::lda_c_chachiyo_mod;
pub use libxc_kernel_lda_1::lda_c_gk72;
pub use libxc_kernel_lda_1::lda_c_gombas;
pub use libxc_kernel_lda_1::lda_c_hl;
pub use libxc_kernel_lda_1::lda_c_lp96;
pub use libxc_kernel_lda_1::lda_c_ml1;
pub use libxc_kernel_lda_1::lda_c_pk09;
pub use libxc_kernel_lda_1::lda_c_pmgb06;
pub use libxc_kernel_lda_1::lda_c_pw;
pub use libxc_kernel_lda_1::lda_c_pw_erf;
pub use libxc_kernel_lda_1::lda_c_pz;
pub use libxc_kernel_lda_1::lda_c_rc04;
pub use libxc_kernel_lda_1::lda_c_rpa;
pub use libxc_kernel_lda_1::lda_c_vwn;
pub use libxc_kernel_lda_1::lda_c_vwn_1;
pub use libxc_kernel_lda_1::lda_c_vwn_2;
pub use libxc_kernel_lda_1::lda_c_vwn_3;
pub use libxc_kernel_lda_1::lda_c_vwn_4;
pub use libxc_kernel_lda_1::lda_c_vwn_rpa;
pub use libxc_kernel_lda_1::lda_c_w20;
pub use libxc_kernel_lda_1::lda_c_wigner;
pub use libxc_kernel_lda_1::lda_k_gds08_worker;
pub use libxc_kernel_lda_1::lda_k_tf;
pub use libxc_kernel_lda_1::lda_k_zlp;
pub use libxc_kernel_lda_1::lda_x;
pub use libxc_kernel_lda_1::lda_x_2d;
pub use libxc_kernel_lda_1::lda_x_erf;
pub use libxc_kernel_lda_1::lda_x_rel;
pub use libxc_kernel_lda_1::lda_x_sloc;
pub use libxc_kernel_lda_1::lda_x_yukawa;
pub use libxc_kernel_lda_1::lda_xc_1d_ehwlrg;
pub use libxc_kernel_lda_1::lda_xc_ksdt;
pub use libxc_kernel_lda_1::lda_xc_teter93;
pub use libxc_kernel_lda_1::lda_xc_tih;
pub use libxc_kernel_lda_1::lda_xc_zlp;

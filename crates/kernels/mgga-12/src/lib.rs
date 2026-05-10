#![allow(clippy::excessive_precision)]
#![allow(clippy::needless_late_init)]
#![allow(clippy::too_many_arguments)]

//! MGGA kernel translations batch 12.

pub mod hyb_mgga_x_m05;
pub mod mgga_c_cc;
pub mod mgga_c_cs;
pub mod mgga_k_csk;
pub mod mgga_k_gea2;
pub mod mgga_k_gea4;
pub mod mgga_k_lk;
pub mod mgga_k_pgslb;
pub mod mgga_k_rda;
pub mod mgga_x_2d_js17;
// pub mod mgga_x_2d_prp10;  // DEFERRED (id 211) — requires xc_bessel_I0/I1 not implemented in libxc-kernel-math.
                              // Comment-out was lost during q06/q08 rebatching; restored per Phase 04-04-SUMMARY.
pub mod mgga_x_gx;
pub mod mgga_x_lta;
pub mod mgga_x_m06l;
pub mod mgga_x_m08;
pub mod mgga_x_ms;
pub mod mgga_x_msb;
pub mod mgga_x_mvs;
pub mod mgga_x_pbe_gx;
pub mod mgga_x_pkzb;
pub mod mgga_x_rlda;
pub mod mgga_x_tau_hcth;
pub mod mgga_x_tb09;
pub mod mgga_x_th;
pub mod mgga_xc_cc06;
pub mod mgga_xc_lp90;
pub mod mgga_xc_zlp;

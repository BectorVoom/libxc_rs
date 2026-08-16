//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 15 (v4rho3sigma_3) CSE chunk 1294/1352 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part15_v4rho3sigma_3_chunk1294(t53994: f64, t53996: f64, t53998: f64, t54000: f64, t54002: f64, t54004: f64, t54006: f64, t54008: f64, t54010: f64, t54012: f64, t54015: f64, t54016: f64) -> f64 {
    let t54018 = t53994 / 32.0_f64 + t53996 / 24.0_f64 + t53998 / 24.0_f64 - t54000 / 192.0_f64 - t54002 / 384.0_f64 + t54004 / 24.0_f64 - t54006 / 48.0_f64 - t54008 / 96.0_f64 + t54010 / 16.0_f64 - t54012 / 48.0_f64 + t54015 + t54016 / 192.0_f64;
    t54018
}

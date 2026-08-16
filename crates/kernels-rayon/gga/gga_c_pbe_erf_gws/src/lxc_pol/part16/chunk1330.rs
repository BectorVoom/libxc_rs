//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 1330/1361 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk1330(t15018: f64, t840: f64, t53896: f64, t54014: f64, t53994: f64, t53996: f64, t53998: f64, t54000: f64, t54002: f64, t54004: f64, t54006: f64, t54008: f64, t54010: f64, t54012: f64, t54016: f64) -> (f64, f64, f64) {
    let t55420 = 7.0_f64 / 144.0_f64 * t840 * t15018;
    let t55421 = 7.0_f64 / 36.0_f64 * t53896;
    let t55432 = 7.0_f64 / 288.0_f64 * t54014;
    let t55434 = t53994 / 16.0_f64 + t53996 / 12.0_f64 + t53998 / 12.0_f64 - t54000 / 96.0_f64 - t54002 / 192.0_f64 + t54004 / 12.0_f64 - t54006 / 24.0_f64 - t54008 / 48.0_f64 + t54010 / 8.0_f64 - t54012 / 24.0_f64 + t55432 + t54016 / 96.0_f64;
    (t55420, t55421, t55434)
}

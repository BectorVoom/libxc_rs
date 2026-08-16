//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 20 (v4rho3sigma_8) CSE chunk 772/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part20_v4rho3sigma_8_chunk772(t1764: f64, t187: f64, t22: f64, t1878: f64, t586: f64, t1778: f64, t633: f64, t198: f64, t2735: f64, t185: f64, t5081: f64, t1903: f64, t720: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t5292 = 1.0_f64 / t187 / t1764;
    let t5293 = t22 * t5292;
    let t5312 = t1878 * t586;
    let t5355 = t633 * t1778;
    let t5357 = t2735 * t198;
    let t5359 = 16.0_f64 / 405.0_f64 * t185 * t5357;
    let t5360 = 0.58774074074074074074e-2_f64 * t5081;
    let t5384 = 2.0_f64 / 9.0_f64 * t720 * t1903;
    (t5293, t5312, t5355, t5359, t5360, t5384)
}

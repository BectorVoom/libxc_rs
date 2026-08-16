//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 1313/1361 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk1313(t53178: f64, t53198: f64, t53230: f64, t14182: f64, t14193: f64, t22379: f64, t2352: f64, t2409: f64, t26604: f64, t3066: f64, t3067: f64, t4227: f64, t52197: f64, t52199: f64, t53174: f64, t53182: f64, t53207: f64, t53212: f64, t53227: f64, t53234: f64, t53238: f64) -> f64 {
    let t55005 = 7.0_f64 / 288.0_f64 * t53178;
    let t55007 = 7.0_f64 / 288.0_f64 * t53198;
    let t55022 = 7.0_f64 / 72.0_f64 * t53230;
    let t55025 = t53174 / 384.0_f64 - t55005 + 5.0_f64 / 384.0_f64 * t53182 - t55007 + t3066 * t2409 * t3067 * t4227 * t2352 / 48.0_f64 - 5.0_f64 / 384.0_f64 * t53207 + 7.0_f64 / 48.0_f64 * t52197 + t53212 / 192.0_f64 - 7.0_f64 / 72.0_f64 * t52199 + t22379 * t14182 / 24.0_f64 + t26604 * t14193 / 48.0_f64 + t53227 / 384.0_f64 + t55022 - t53234 / 24.0_f64 + t53238 / 192.0_f64;
    t55025
}

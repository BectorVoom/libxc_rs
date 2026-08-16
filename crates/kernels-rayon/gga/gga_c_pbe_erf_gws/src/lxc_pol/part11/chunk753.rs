//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 753/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk753(t8012: f64, t8014: f64, t12361: f64, t85: f64, t10257: f64, t10259: f64, t8016: f64, t4688: f64, t4711: f64, t4714: f64, t4718: f64, t4807: f64, t4815: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t12369 = 24.0_f64 * t8012;
    let t12370 = 36.0_f64 * t8014;
    let t12371 = t12361 * t85;
    let t12372 = 0.19751789702565206229e-1_f64 * t12371;
    let t12373 = 12.0_f64 * t10257;
    let t12374 = 12.0_f64 * t10259;
    let t12375 = 0.17544670192365612213e1_f64 * t8016;
    let t12376 = t4807 - t4815 + t4688 + t4711 - t4714 - t4718 - t12369 + t12370 + t12372 - t12373 - t12374 - t12375;
    (t12369, t12370, t12371, t12372, t12373, t12374, t12375, t12376)
}

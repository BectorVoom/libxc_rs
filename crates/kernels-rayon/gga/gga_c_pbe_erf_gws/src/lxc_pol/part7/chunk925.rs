//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 925/1242 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk925(t1648: f64, t5395: f64, t1627: f64, t5403: f64, t1642: f64, t212: f64, t22: f64, t16972: f64, t219: f64, t16973: f64, t639: f64, t1656: f64, t5406: f64) -> (f64, f64, f64, f64) {
    let t17316 = 16.0_f64 / 5.0_f64 * t1648 * t5395;
    let t17318 = 128.0_f64 / 81.0_f64 * t1627 * t5403;
    let t17321 = t22 / t212 / t1642;
    let t17322 = t219 * t16972;
    let t17326 = 352.0_f64 / 243.0_f64 * t639 * t17321 * t17322 * t16973;
    let t17328 = 8.0_f64 / 15.0_f64 * t5406 * t1656;
    (t17316, t17318, t17326, t17328)
}

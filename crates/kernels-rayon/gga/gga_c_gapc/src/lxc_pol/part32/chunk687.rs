//! GGA_C_GAPC lxc pol — lxc_pol part 32 (v4rho2sigma2_11) CSE chunk 687/1311 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part32_v4rho2sigma2_11_chunk687(t786: f64, t952: f64, t3327: f64, t7158: f64, t875: f64, t2761: f64, t959: f64, t967: f64, t935: f64, t4978: f64, t961: f64, t2801: f64, t329: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t7324 = t952 * t786;
    let t7325 = t3327 * t7324;
    let t7329 = t7158 * t875;
    let t7330 = t3327 * t7329;
    let t7333 = t2761 * t959;
    let t7334 = t967 * t786;
    let t7335 = t7333 * t7334;
    let t7371 = t935 * t875;
    let t7375 = t961 * t4978;
    let t7389 = t329 * t2801;
    (t7325, t7330, t7333, t7335, t7371, t7375, t7389)
}

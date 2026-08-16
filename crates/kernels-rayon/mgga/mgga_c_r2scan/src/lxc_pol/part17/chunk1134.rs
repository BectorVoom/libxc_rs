//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 1134/1293 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk1134(t11336: f64, t2850: f64, t3270: f64, t3493: f64, t983: f64, t11002: f64, t3719: f64, t481: f64, t910: f64, t14402: f64, t986: f64, t39355: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t41298 = t3270 * t11336 * t2850;
    let t41326 = t3493 * t983;
    let t41327 = t11002 * t41326;
    let t41336 = t3719 * t481;
    let t41337 = t3270 * t41336;
    let t41343 = t3493 * t910;
    let t41344 = t3270 * t41343;
    let t41347 = t14402 * t986;
    let t41348 = t3270 * t41347;
    let t41352 = 0.28565981518604370584e-1_f64 * t39355;
    (t41298, t41327, t41337, t41344, t41348, t41352)
}

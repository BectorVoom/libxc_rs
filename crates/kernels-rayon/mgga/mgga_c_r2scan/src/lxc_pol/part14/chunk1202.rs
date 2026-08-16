//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 1202/1276 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk1202(t3270: f64, t41336: f64, t10667: f64, t11336: f64, t37327: f64, t40566: f64, t3493: f64, t910: f64, t14402: f64, t986: f64, t3269: f64, t39355: f64) -> (f64, f64, f64, f64, f64) {
    let t41337 = t3270 * t41336;
    let t41339 = 3.0_f64 / 2.0_f64 * t10667 * t41337;
    let t41342 = 15.0_f64 / 8.0_f64 * t37327 * t11336 * t40566;
    let t41343 = t3493 * t910;
    let t41344 = t3270 * t41343;
    let t41346 = 3.0_f64 / 2.0_f64 * t10667 * t41344;
    let t41347 = t14402 * t986;
    let t41348 = t3270 * t41347;
    let t41350 = t3269 * t41348 / 2.0_f64;
    let t41352 = 0.28565981518604370584e-1_f64 * t39355;
    (t41339, t41342, t41346, t41350, t41352)
}

//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 813/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk813(t7592: f64, t7523: f64, t7525: f64, t7527: f64, t7531: f64, t7535: f64, t7541: f64, t7547: f64, t7550: f64, t7576: f64, t7580: f64, t7583: f64, t7596: f64, t7599: f64) -> f64 {
    let t7656 = 0.36514074074074074075e0_f64 * t7592;
    let t7657 = 0.93011851851851851854e0_f64 * t7523;
    let t7662 = 0.5477111111111111111e-1_f64 * t7576 - 0.36514074074074074075e-1_f64 * t7580 - 0.82156666666666666667e-1_f64 * t7583 - 0.39862222222222222223e0_f64 * t7525 + 0.29896666666666666667e0_f64 * t7531 + 0.19931111111111111111e0_f64 * t7527 - 0.33218518518518518518e0_f64 * t7535 - 0.29896666666666666667e0_f64 * t7550 - t7656 - t7657 - 0.82156666666666666668e-1_f64 * t7596 + 0.49293999999999999999e0_f64 * t7599 - 0.59793333333333333333e0_f64 * t7541 + 0.17938e1_f64 * t7547;
    t7662
}

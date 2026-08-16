//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 821/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk821(t7592: f64, t7523: f64, t7525: f64, t7527: f64, t7531: f64, t7535: f64, t7541: f64, t7547: f64, t7550: f64, t7576: f64, t7580: f64, t7583: f64, t7596: f64, t7599: f64) -> f64 {
    let t7786 = 0.46308888888888888888e0_f64 * t7592;
    let t7787 = 0.16068111111111111111e1_f64 * t7523;
    let t7792 = 0.69463333333333333335e-1_f64 * t7576 - 0.46308888888888888889e-1_f64 * t7580 - 0.104195e0_f64 * t7583 - 0.68863333333333333332e0_f64 * t7525 + 0.51647499999999999999e0_f64 * t7531 + 0.34431666666666666666e0_f64 * t7527 - 0.57386111111111111112e0_f64 * t7535 - 0.516475e0_f64 * t7550 - t7786 - t7787 - 0.104195e0_f64 * t7596 + 0.62517e0_f64 * t7599 - 0.103295e1_f64 * t7541 + 0.309885e1_f64 * t7547;
    t7792
}

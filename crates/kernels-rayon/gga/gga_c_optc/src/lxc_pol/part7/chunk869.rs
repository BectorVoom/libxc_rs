//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 869/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk869(t7523: f64, t7525: f64, t7527: f64, t7531: f64, t7535: f64, t7550: f64, t7571: f64, t7573: f64, t7576: f64, t7580: f64, t7583: f64, t8320: f64) -> f64 {
    let t8321 = 0.60319259259259259259e1_f64 * t7523;
    let t8332 = -t8321 - 0.4105e-2_f64 * t7571 + 0.2463e-2_f64 * t7573 + 0.821e-3_f64 * t7576 - 0.54733333333333333333e-3_f64 * t7580 - 0.12315e-2_f64 * t7583 - 0.2585111111111111111e1_f64 * t7525 + 0.19388333333333333333e1_f64 * t7531 + 0.12925555555555555555e1_f64 * t7527 - 0.21542592592592592592e1_f64 * t7535 - 0.19388333333333333333e1_f64 * t7550;
    let t8333 = t8320 + t8332;
    t8333
}

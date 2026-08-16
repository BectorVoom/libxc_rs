//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 810/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk810(t7604: f64, t828: f64, t837: f64, t845: f64, t7523: f64, t7525: f64, t7527: f64, t7529: f64, t7531: f64, t7535: f64, t7538: f64, t7541: f64, t7544: f64, t7547: f64, t7550: f64) -> (f64, f64, f64) {
    let t7606 = t828 * t7604 * t837;
    let t7608 = 0.58482233974552040708e0_f64 * t845 * t7606;
    let t7609 = 0.28842592592592592592e-1_f64 * t7523;
    let t7620 = -t7609 - 0.12361111111111111111e-1_f64 * t7525 + 0.61805555555555555556e-2_f64 * t7527 - 0.18541666666666666667e-1_f64 * t7529 + 0.92708333333333333334e-2_f64 * t7531 - 0.10300925925925925926e-1_f64 * t7535 + 0.37083333333333333333e-1_f64 * t7538 - 0.18541666666666666666e-1_f64 * t7541 - 0.55625000000000000001e-1_f64 * t7544 + 0.55625000000000000001e-1_f64 * t7547 - 0.92708333333333333333e-2_f64 * t7550;
    (t7606, t7608, t7620)
}

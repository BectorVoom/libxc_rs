//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 968/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk968(t8639: f64, t8642: f64, t8587: f64, t8589: f64, t8591: f64, t8622: f64, t8625: f64, t8628: f64, t8630: f64, t8632: f64, t8636: f64, t8593: f64, t8595: f64, t8598: f64, t8601: f64, t8603: f64, t8606: f64, t8609: f64, t8651: f64, t8654: f64, t8657: f64, t8660: f64) -> (f64, f64) {
    let t9268 = 0.60319259259259259259e1_f64 * t8639;
    let t9269 = 0.54733333333333333333e-2_f64 * t8642;
    let t9279 = -t9268 - t9269 - 0.21542592592592592592e1_f64 * t8622 - 0.19388333333333333333e1_f64 * t8625 - 0.4105e-2_f64 * t8628 + 0.2463e-2_f64 * t8630 + 0.821e-3_f64 * t8632 - 0.54733333333333333333e-3_f64 * t8636 - 0.12315e-2_f64 * t8587 - 0.2585111111111111111e1_f64 * t8589 + 0.19388333333333333333e1_f64 * t8591;
    let t9291 = 0.12925555555555555555e1_f64 * t8593 - 0.4926e-2_f64 * t8595 + 0.2463e-2_f64 * t8598 - 0.12315e-2_f64 * t8651 - 0.7389e-2_f64 * t8601 + 0.7389e-2_f64 * t8654 - 0.38776666666666666665e1_f64 * t8603 + 0.77553333333333333331e1_f64 * t8606 - 0.38776666666666666665e1_f64 * t8657 - 0.11633e2_f64 * t8609 + 0.11633e2_f64 * t8660;
    (t9279, t9291)
}

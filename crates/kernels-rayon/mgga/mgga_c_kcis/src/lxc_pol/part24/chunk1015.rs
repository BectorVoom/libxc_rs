//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 1015/1322 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk1015(t26623: f64, t7589: f64, t7580: f64, t2140: f64, t334: f64, t9232: f64, t26457: f64, t26595: f64, t26598: f64, t26600: f64, t26603: f64, t26605: f64, t26608: f64, t26612: f64, t26616: f64, t26618: f64) -> f64 {
    let t26624 = t7589 * t26623;
    let t26626 = t7580 * t26623;
    let t26629 = t9232 * t334 * t2140;
    let t26631 = 0.30952962962962962962e-1_f64 * t26457 - 0.185671721767578125e-4_f64 * t26595 - 0.32435763888888888888e-2_f64 * t26598 - 0.32435763888888888888e-2_f64 * t26600 + 0.13901041666666666667e-2_f64 * t26603 + 0.13901041666666666667e-2_f64 * t26605 + 0.18550940104166666667e-3_f64 * t26608 + 0.92754700520833333333e-4_f64 * t26612 + 0.69505208333333333333e-3_f64 * t26616 + 0.69505208333333333333e-3_f64 * t26618 - 0.13901041666666666667e-2_f64 * t26624 - 0.18550940104166666667e-3_f64 * t26626 - 0.69505208333333333333e-3_f64 * t26629;
    t26631
}

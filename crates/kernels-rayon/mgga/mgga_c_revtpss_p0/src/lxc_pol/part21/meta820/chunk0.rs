//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 3027/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3027(t342: f64, t378: f64, t43536: f64, t11631: f64, t43350: f64, t16558: f64, t989: f64, t1071: f64, t12166: f64, t12077: f64, t11247: f64, t1678: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t55569 = t342 * t43536 * t378;
    let t55570 = t43350 * t11631;
    let t55575 = t989 * t16558;
    let t55579 = t342 * t12166 * t1071;
    let t55583 = t342 * t12077 * t1071;
    let t55586 = t1678 * t11247;
    (t55569, t55570, t55575, t55579, t55583, t55586)
}

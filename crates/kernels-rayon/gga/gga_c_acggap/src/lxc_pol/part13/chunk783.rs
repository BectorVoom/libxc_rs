//! GGA_C_ACGGAP lxc pol — lxc_pol part 13 (v4rho3sigma_5) CSE chunk 783/1213 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part13_v4rho3sigma_5_chunk783(t1089: f64, t1459: f64, t8484: f64, t598: f64, t355: f64, t513: f64, t7458: f64, t1980: f64, t1988: f64, t2294: f64, t2288: f64, t3201: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t8486 = t1089 * t1459 * t8484;
    let t8487 = t598 * t8486;
    let t8489 = t355 * t513;
    let t8491 = t7458 * t1459 * t8489;
    let t8492 = t1980 * t8491;
    let t8494 = t1988 * t2294;
    let t8497 = t1089 * t3201 * t2288;
    (t8486, t8487, t8489, t8491, t8492, t8494, t8497)
}

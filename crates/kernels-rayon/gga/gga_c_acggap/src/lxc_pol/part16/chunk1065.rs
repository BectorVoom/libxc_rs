//! GGA_C_ACGGAP lxc pol — lxc_pol part 16 (v4rho3sigma_8) CSE chunk 1065/1223 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part16_v4rho3sigma_8_chunk1065(t1089: f64, t3201: f64, t598: f64, t9541: f64, t1083: f64, t137: f64, t5784: f64, t1772: f64, t1980: f64, t355: f64, t7458: f64, t1841: f64, t7712: f64) -> (f64, f64, f64, f64) {
    let t38805 = t598 * t1089 * t3201 * t9541;
    let t38810 = t598 * t1089 * t1083 * t137 * t5784;
    let t38815 = t1980 * t7458 * t1083 * t355 * t1772;
    let t38817 = t7712 * t1841;
    (t38805, t38810, t38815, t38817)
}

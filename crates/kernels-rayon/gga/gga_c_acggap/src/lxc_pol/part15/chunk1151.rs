//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 1151/1278 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk1151(t13287: f64, t31057: f64, t38857: f64, t1181: f64, t5651: f64, t599: f64, t8463: f64, t5572: f64, t7351: f64, t7575: f64, t2016: f64, t9618: f64) -> (f64, f64, f64, f64) {
    let t39914 = t31057 * t13287 * t38857;
    let t39919 = t8463 * t1181 * t599 * t5651;
    let t39923 = t7575 * t1181 * t7351 * t5572;
    let t39925 = t2016 * t9618;
    (t39914, t39919, t39923, t39925)
}

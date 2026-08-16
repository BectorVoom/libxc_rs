//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 741/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk741(t3181: f64, t439: f64, t442: f64, t446: f64, t8113: f64, t19: f64, t8915: f64, t123: f64, t4434: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t9166 = 1.0_f64 / t3181 / t439;
    let t9167 = t9166 * t442;
    let t9168 = t9167 * t446;
    let t9169 = t9168 * t8113;
    let t9170 = t8915 * t19;
    let t9171 = t9170 * t123;
    let t9175 = t4434 * t8113;
    (t9166, t9167, t9168, t9169, t9170, t9171, t9175)
}

//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1235/1478 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1235(t464: f64, t6673: f64, t132: f64, t137: f64, t477: f64, t1629: f64, t6734: f64, t12245: f64, t12248: f64, t12259: f64, t161: f64, t166: f64, t2088: f64, t4801: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t16267 = t6673 * t464;
    let t16271 = t132 * t137 * t16267 * t477 / 15.0_f64;
    let t16275 = t132 * t137 * t6734 * t1629 / 30.0_f64;
    let t16276 = 4.0_f64 / 135.0_f64 * t12245;
    let t16277 = 2.0_f64 / 45.0_f64 * t12248;
    let t16278 = 4.0_f64 / 45.0_f64 * t12259;
    let t16282 = 2.0_f64 / 15.0_f64 * t161 * t166 * t4801 * t2088;
    (t16271, t16275, t16276, t16277, t16278, t16282)
}

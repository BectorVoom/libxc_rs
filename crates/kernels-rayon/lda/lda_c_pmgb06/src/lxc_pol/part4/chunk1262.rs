//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1262/1478 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1262(t12831: f64, t9762: f64, t9765: f64, t1554: f64, t161: f64, t2554: f64, t517: f64, t6831: f64, t166: f64, t529: f64, t1586: f64, t6230: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t16589 = 4.0_f64 / 135.0_f64 * t12831;
    let t16590 = 8.0_f64 / 405.0_f64 * t9762;
    let t16591 = 8.0_f64 / 405.0_f64 * t9765;
    let t16593 = t161 * t1554 * t2554;
    let t16594 = t16593 / 135.0_f64;
    let t16595 = t6831 * t517;
    let t16599 = t161 * t166 * t16595 * t529 / 15.0_f64;
    let t16603 = t161 * t166 * t6230 * t1586 / 30.0_f64;
    (t16589, t16590, t16591, t16594, t16599, t16603)
}

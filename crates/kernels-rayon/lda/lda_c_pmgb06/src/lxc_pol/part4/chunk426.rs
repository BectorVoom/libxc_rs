//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 426/1478 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk426(t1586: f64, t518: f64, t166: f64, t161: f64, t152: f64, t463: f64) -> (f64, f64, f64, f64) {
    let t1587 = t518 * t1586;
    let t1588 = t166 * t1587;
    let t1590 = t161 * t1588 / 30.0_f64;
    let t1592 = 1.0_f64 / t463 / t152;
    (t1587, t1588, t1590, t1592)
}

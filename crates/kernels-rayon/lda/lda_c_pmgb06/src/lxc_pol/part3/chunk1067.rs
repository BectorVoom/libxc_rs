//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1067/1239 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk1067(t132: f64, t137: f64, t822: f64, t9590: f64, t1436: f64, t1439: f64, t2010: f64, t332: f64, t1423: f64, t4767: f64, t1558: f64, t442: f64) -> (f64, f64, f64, f64) {
    let t12672 = t132 * t137 * t9590 * t822 / 30.0_f64;
    let t12676 = 2.0_f64 / 9.0_f64 * t2010 * t1436 * t1439 * t332;
    let t12677 = t1423 * t4767;
    let t12678 = 2.0_f64 / 5.0_f64 * t12677;
    let t12682 = 4.0_f64 / 15.0_f64 * t2010 * t442 * t1558 * t332;
    (t12672, t12676, t12678, t12682)
}

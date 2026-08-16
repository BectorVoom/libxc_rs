//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 915/1267 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk915(t11395: f64, t2060: f64, t5796: f64, t5799: f64, t947: f64, t5802: f64, t2221: f64, t410: f64, t360: f64, t138: f64, t53: f64, t3631: f64, t783: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t11396 = 2.93808_f64 * t11395;
    let t11398 = t5796 * t2060;
    let t11400 = t5799 * t947;
    let t11401 = 1.9486833333333333_f64 * t11400;
    let t11402 = t5802 * t2060;
    let t11404 = t410 * t2221;
    let t11405 = t360 * t11404;
    let t11406 = 2.0_f64 / 3.0_f64 * t11405;
    let t11407 = t53 * t138;
    let t11465 = t783 * t3631;
    (t11396, t11398, t11401, t11402, t11404, t11406, t11407, t11465)
}

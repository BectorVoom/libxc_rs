//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 910/1239 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk910(t1431: f64, t3220: f64, t1441: f64, t3213: f64, t1423: f64, t3255: f64, t3263: f64, t1179: f64, t161: f64, t165: f64, t177: f64, t2953: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t10103 = t3220 * t1431;
    let t10105 = t3220 * t1441;
    let t10109 = t3213 * t1431;
    let t10111 = t1423 * t3255;
    let t10113 = t1423 * t3263;
    let t10134 = 28.0_f64 / 1215.0_f64 * t161 * t1179 * t165 * t177;
    let t10137 = t1423 * t2953;
    (t10103, t10105, t10109, t10111, t10113, t10134, t10137)
}

//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 282/1239 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk282(t125: f64, t52: f64, t934: f64, t62: f64, t97: f64, t315: f64, t409: f64, t55: f64, t623: f64, t30: f64, t410: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t936 = t934 * t125 * t52;
    let t939 = 1.0_f64 / t62;
    let t940 = t939 * t97;
    let t941 = t934 * t315;
    let t942 = t940 * t941;
    let t944 = t55 * t409;
    let t945 = t623 * t944;
    let t947 = t30 * t410;
    (t936, t939, t940, t941, t942, t944, t945, t947)
}

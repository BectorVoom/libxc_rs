//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 282/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk282<F: Float>(t125: F, t52: F, t934: F, t62: F, t97: F, t315: F, t409: F, t55: F, t623: F, t30: F, t410: F) -> (F, F, F, F, F, F, F, F) {
    let t936 = t934 * t125 * t52;
    let t939 = F::cast_from(1.0_f64) / t62;
    let t940 = t939 * t97;
    let t941 = t934 * t315;
    let t942 = t940 * t941;
    let t944 = t55 * t409;
    let t945 = t623 * t944;
    let t947 = t30 * t410;
    (t936, t939, t940, t941, t942, t944, t945, t947)
}

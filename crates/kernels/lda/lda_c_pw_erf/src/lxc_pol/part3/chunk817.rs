//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 817/1138 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk817<F: Float>(t9133: F, t137: F, t142: F, t2777: F, t440: F, t1549: F, t2810: F, t169: F, t301: F, t3196: F, t717: F, t3251: F, t2775: F, t450: F, t2778: F, t147: F, t159: F, t285: F, t3165: F) -> (F, F, F, F, F, F) {
    let t9134 = 1.0 / t9133;
    let t9135 = t9134 * t137;
    let t9138 = t9135 * t142 * t2777 * t440;
    let t9141 = t1549 * t2810;
    let t9146 = t169 * t717 * t3196 * t301;
    let t9148 = t142 * t3251;
    let t9156 = t2775 * t450;
    let t9157 = t9156 * t2778;
    let t9163 = 1.0943113336969376e-06 * t3165 * t147 * t159 * t285;
    (t9138, t9141, t9146, t9148, t9157, t9163)
}

//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 725/1265 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk725<F: Float>(t5071: F, t5139: F, t5138: F, t3074: F, t3077: F, t3149: F, t3151: F, t3153: F, t3156: F, t3158: F, t3165: F, t3182: F, t3201: F, t3214: F, t3221: F, t3224: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t5140 = t5139 * t5071;
    let t5142 = 2.0 / 27.0 * t5138 * t5140;
    let t5143 = t3074 / 45.0;
    let t5144 = t3077 / 45.0;
    let t5145 = t3149 / 45.0;
    let t5146 = 2.0 / 45.0 * t3151;
    let t5147 = 2.0 / 45.0 * t3153;
    let t5148 = 2.0 / 135.0 * t3156;
    let t5149 = t3158 / 45.0;
    let t5150 = 4.0 / 135.0 * t3165;
    let t5151 = 4.0 / 135.0 * t3182;
    let t5152 = 4.0 / 135.0 * t3201;
    let t5153 = 4.0 / 405.0 * t3214;
    let t5154 = 4.0 / 135.0 * t3221;
    let t5155 = 4.0 / 405.0 * t3224;
    (t5140, t5142, t5143, t5144, t5145, t5146, t5147, t5148, t5149, t5150, t5151, t5152, t5153, t5154, t5155)
}

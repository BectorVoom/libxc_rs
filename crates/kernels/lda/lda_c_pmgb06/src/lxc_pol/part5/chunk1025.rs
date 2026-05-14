//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1025/1097 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk1025<F: Float>(t17025: F, t17030: F, t17035: F, t17041: F, t17054: F, t17059: F, t17061: F, t17066: F, t19316: F, t19319: F, t19322: F, t19324: F, t19326: F, t19334: F, t19338: F, t19342: F, t19346: F, t19351: F, t19356: F, t19360: F) -> (F,) {
    let t21117 = -0.64785 * t19316 + 0.4319 * t19319 - 0.11997222222222222 * t19322 + 0.07198333333333333 * t19324 + 0.011997222222222222 * t19326 - 0.007407407407407408 * t17025 + 0.044444444444444446 * t17030 - 0.022222222222222223 * t17035 + 0.013333333333333334 * t17041 + 0.035991666666666665 * t17054 + 0.09597777777777777 * t17059 - 0.03199259259259259 * t17061 - 0.047988888888888886 * t17066 + 0.8638 * t19334 + 1.2957 * t19338 + 0.21595 * t19342 - 0.4319 * t19346 + 0.07198333333333333 * t19351 - 0.023994444444444443 * t19356 - 0.07198333333333333 * t19360;
    (t21117,)
}

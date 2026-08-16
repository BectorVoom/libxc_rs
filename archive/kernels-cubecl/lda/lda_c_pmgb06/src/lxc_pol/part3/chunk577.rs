//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 577/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk577<F: Float>(t3010: F, t3092: F, t3090: F, t36: F, t1437: F, t5: F) -> (F, F, F, F) {
    let t3093 = t3092 * t3010;
    let t3094 = t3090 * t3093;
    let t3095 = t36 * t3094;
    let t3098 = F::cast_from(1.0_f64) / t1437 / t5;
    (t3093, t3094, t3095, t3098)
}

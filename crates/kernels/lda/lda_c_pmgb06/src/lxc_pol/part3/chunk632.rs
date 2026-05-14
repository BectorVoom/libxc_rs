//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 632/1081 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk632<F: Float>(t3072: F, t3075: F, t3078: F, t3124: F, t3126: F, t3136: F, t3138: F, t3148: F, t3150: F, t3152: F, t3154: F, t3157: F, t3159: F, t3164: F, t3166: F, t3168: F) -> (F,) {
    let t4155 = -t3072 + t3075 + t3078 + t3124 + t3126 + t3136 + t3138 + t3148 + t3150 + t3152 + t3154 - t3157 + t3159 + t3164 + t3166 - t3168;
    (t4155,)
}

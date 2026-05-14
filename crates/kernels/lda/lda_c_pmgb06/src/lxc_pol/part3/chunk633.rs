//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 633/1081 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk633<F: Float>(t3171: F, t3176: F, t3179: F, t3181: F, t3183: F, t3185: F, t3188: F, t3193: F, t3197: F, t3200: F, t3202: F, t3206: F, t3212: F, t3215: F, t3219: F, t3222: F, t3225: F) -> (F,) {
    let t4156 = -t3171 - t3176 + t3179 + t3181 + t3183 - t3185 - t3188 - t3193 + t3197 + t3200 - t3202 + t3206 + t3212 - t3215 + t3219 + t3222 - t3225;
    (t4156,)
}

//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 744/1157 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk744<F: Float>(t7376: F, t87: F, t40: F, t3139: F, t3151: F, t3157: F, t3159: F, t3162: F, t3168: F, t3170: F, t3174: F, t3176: F, t7380: F, t7381: F, t7382: F, t7349: F, t7351: F, t7379: F) -> (F, F, F) {
    let t7383 = t7376 * t87;
    let t7384 = t40 * t7383;
    let t7385 = -t3139 + t3151 - t7380 - t3157 - t3159 - t7381 - t3162 - t3168 + t7382 + t7384 + t3170 + t3174 + t3176;
    let t7387 = t7349 + t7351 + t7379 + t7385;
    (t7383, t7384, t7387)
}

//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 579/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk579<F: Float>(t390: F, t960: F, t40: F, t3168: F, t3170: F, t3172: F, t3174: F, t3176: F, t3178: F, t3180: F, t3182: F, t3184: F, t3186: F, t3188: F, t3190: F) -> (F, F, F) {
    let t3191 = t960 * t390;
    let t3192 = t40 * t3191;
    let t3193 = F::new(3.0) * t3192;
    let t3194 = -t3168 + t3170 + t3172 - t3174 + t3176 - t3178 + t3180 + t3182 - t3184 + t3186 - t3188 + t3190 + t3193;
    (t3191, t3192, t3194)
}

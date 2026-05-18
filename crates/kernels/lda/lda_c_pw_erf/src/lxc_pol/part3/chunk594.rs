//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 594/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk594<F: Float>(t133: F, t3227: F, t3230: F, t1568: F, t411: F, t3219: F, t1870: F, t1871: F, t3224: F, t3253: F, t3269: F, t3271: F, t3275: F, t3277: F, t3280: F, t3284: F, t3302: F, t3305: F, t3322: F, t3325: F, t3348: F) -> (F, F, F, F, F) {
    let t3349 = t133 * t3227;
    let t3351 = t133 * t3230;
    let t3357 = t411 * t1568;
    let t3361 = t133 * t3219;
    let t3363 = -t3348 - F::new(2.2990066666666666) * t3349 + F::new(1.724255) * t3351 - F::new(1.724255) * t133 * t3253 - F::new(20.69106) * t133 * t3224 + F::new(15.518295) * t1870 * t1871 * t3357 - F::new(5.172765) * t3361 - t3284 + t3269 + t3280 + t3271 - t3275 - t3277 - t3322 - t3302 + t3325 - t3305;
    (t3349, t3351, t3357, t3361, t3363)
}

//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1001/1138 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk1001<F: Float>(t10322: F, t13396: F, t13398: F, t13400: F, t13402: F, t13403: F, t13405: F, t13407: F, t13408: F, t13409: F, t13410: F, t13411: F, t13412: F, t10326: F, t10350: F, t10361: F) -> (F, F, F, F, F) {
    let t13413 = 8.0 / 27.0 * t10322;
    let t13414 = t13396 + t13398 + t13400 + t13402 + t13403 - t13405 + t13407 - t13408 - t13409 - t13410 - t13411 + t13412 + t13413;
    let t13415 = 16.0 / 15.0 * t10326;
    let t13416 = 16.0 / 45.0 * t10350;
    let t13417 = 16.0 / 135.0 * t10361;
    (t13413, t13414, t13415, t13416, t13417)
}

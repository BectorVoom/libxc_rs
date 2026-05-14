//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1076/1157 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1076<F: Float>(t1446: F, t7475: F, t13925: F, t15108: F, t15109: F, t15111: F, t22350: F, t22352: F, t22354: F, t22358: F, t22361: F, t22362: F, t22363: F, t22367: F, t1472: F, t7479: F) -> (F, F, F) {
    let t22369 = 8.0 / 15.0 * t1446 * t7475;
    let t22370 = t22350 + t22352 + t22354 + t22358 + t22361 - t13925 - t22362 - t22363 - t15108 - 2.0 / 3.0 * t15109 + 8.0 / 27.0 * t15111 - t22367 + t22369;
    let t22372 = 8.0 / 15.0 * t1472 * t7479;
    (t22369, t22370, t22372)
}

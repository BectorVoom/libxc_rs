//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1223/1265 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1223<F: Float>(t17371: F, t17373: F, t17375: F, t17377: F, t17380: F, t17384: F, t17386: F, t17389: F, t17392: F, t17395: F, t17398: F, t17402: F, t17407: F, t17410: F, t17414: F, t10134: F, t17416: F, t17418: F, t17421: F, t17424: F, t17430: F, t17433: F, t17434: F, t17438: F, t17440: F, t17444: F, t17448: F, t17452: F, t17455: F, t17460: F) -> (F, F) {
    let t18302 = t17371 - t17373 - t17375 - t17377 - t17380 - t17384 + t17386 + t17389 + t17392 + t17395 + t17398 - t17402 + t17407 - t17410 + t17414;
    let t18303 = -t10134 + t17416 - t17418 - t17421 - t17424 - t17430 - t17433 + t17434 + t17438 + t17440 + t17444 + t17448 + t17452 - t17455 - t17460;
    (t18302, t18303)
}

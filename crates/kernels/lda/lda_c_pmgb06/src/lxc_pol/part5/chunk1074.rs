//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1074/1097 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk1074<F: Float>(t20334: F, t20337: F, t20338: F, t20340: F, t20343: F, t20346: F, t20353: F, t20355: F, t20359: F, t20361: F, t20364: F, t20367: F, t20369: F, t20372: F, t20374: F, t20376: F, t20378: F, t20380: F, t20382: F, t20386: F, t20390: F, t20394: F, t20397: F, t20400: F) -> (F, F) {
    let t21988 = -t20334 - t20337 + t20338 + t20340 - t20343 + t20346 + t20353 - t20355 + t20359 + t20361 + t20364 + t20367;
    let t21990 = t20369 + t20372 + t20374 + t20376 + t20378 + t20380 + t20382 + t20386 + t20390 + t20394 + t20397 - t20400;
    (t21988, t21990)
}

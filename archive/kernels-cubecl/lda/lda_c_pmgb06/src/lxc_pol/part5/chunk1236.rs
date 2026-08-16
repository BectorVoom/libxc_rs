//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1236/1267 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk1236<F: Float>(t20369: F, t20372: F, t20374: F, t20376: F, t20378: F, t20380: F, t20382: F, t20386: F, t20390: F, t20394: F, t20397: F, t20400: F) -> F {
    let t21990 = t20369 + t20372 + t20374 + t20376 + t20378 + t20380 + t20382 + t20386 + t20390 + t20394 + t20397 - t20400;
    t21990
}

//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1404/1478 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1404<F: Float>(t16364: F, t16367: F, t16372: F, t16373: F, t16374: F, t16375: F, t16379: F, t16381: F, t16384: F, t16385: F, t16389: F, t16390: F, t16396: F, t16398: F, t16400: F) -> F {
    let t18232 = t16364 + t16367 - t16372 - t16373 - t16374 - t16375 - t16379 + t16381 + t16384 - t16385 - t16389 + t16390 - t16396 - t16398 - t16400;
    t18232
}

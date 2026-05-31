//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1406/1478 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1406<F: Float>(t12657: F, t16429: F, t16431: F, t16433: F, t16438: F, t16439: F, t16440: F, t16441: F, t16443: F, t16445: F, t16449: F, t16453: F, t16456: F, t16458: F, t16463: F) -> F {
    let t18236 = -t16429 - t16431 - t16433 + t16438 - t16439 + t16440 + t16441 + t16443 + t16445 - t16449 - t16453 + t16456 + t16458 + t16463 - F::cast_from(16.0_f64) / F::cast_from(405.0_f64) * t12657;
    t18236
}

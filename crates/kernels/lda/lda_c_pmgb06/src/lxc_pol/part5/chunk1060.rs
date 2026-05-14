//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1060/1097 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk1060<F: Float>(t11861: F, t11867: F, t19434: F, t19436: F, t19438: F, t19440: F, t19441: F, t19442: F, t9340: F, t9342: F, t9345: F, t9348: F, t19447: F, t19449: F, t19451: F, t19458: F, t19461: F, t19463: F, t19466: F, t19469: F, t19474: F, t19478: F, t19479: F) -> (F, F) {
    let t21904 = -t19434 + t19436 + t19438 + t19440 + 0.09973633333333333 * t9340 - 0.06649088888888889 * t9342 - t9345 + t9348 + t19441 - t11861 - t19442 - t11867;
    let t21908 = -t19447 + t19449 + t19451 - t19458 + t19461 + t19463 + t19466 + t19469 + t19474 + t19478 + t19479;
    (t21904, t21908)
}

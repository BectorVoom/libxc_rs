//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1215/1267 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk1215<F: Float>(t19447: F, t19449: F, t19451: F, t19458: F, t19461: F, t19463: F, t19466: F, t19469: F, t19474: F, t19478: F, t19479: F, t11882: F, t19480: F, t19481: F, t19482: F, t19485: F, t19488: F, t19493: F, t19497: F, t19498: F, t19499: F, t19504: F, t19507: F) -> (F, F) {
    let t21908 = -t19447 + t19449 + t19451 - t19458 + t19461 + t19463 + t19466 + t19469 + t19474 + t19478 + t19479;
    let t21909 = t19480 - t19481 - t19482 - t19485 - t19488 + t19493 - t19497 + t19498 + t19499 + t19504 + t11882 - t19507;
    (t21908, t21909)
}

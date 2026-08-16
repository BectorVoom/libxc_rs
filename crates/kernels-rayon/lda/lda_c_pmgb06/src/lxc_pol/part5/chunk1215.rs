//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1215/1267 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk1215(t19447: f64, t19449: f64, t19451: f64, t19458: f64, t19461: f64, t19463: f64, t19466: f64, t19469: f64, t19474: f64, t19478: f64, t19479: f64, t11882: f64, t19480: f64, t19481: f64, t19482: f64, t19485: f64, t19488: f64, t19493: f64, t19497: f64, t19498: f64, t19499: f64, t19504: f64, t19507: f64) -> (f64, f64) {
    let t21908 = -t19447 + t19449 + t19451 - t19458 + t19461 + t19463 + t19466 + t19469 + t19474 + t19478 + t19479;
    let t21909 = t19480 - t19481 - t19482 - t19485 - t19488 + t19493 - t19497 + t19498 + t19499 + t19504 + t11882 - t19507;
    (t21908, t21909)
}

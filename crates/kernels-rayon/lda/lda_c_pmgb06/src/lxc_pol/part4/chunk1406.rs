//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1406/1478 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1406(t12657: f64, t16429: f64, t16431: f64, t16433: f64, t16438: f64, t16439: f64, t16440: f64, t16441: f64, t16443: f64, t16445: f64, t16449: f64, t16453: f64, t16456: f64, t16458: f64, t16463: f64) -> f64 {
    let t18236 = -t16429 - t16431 - t16433 + t16438 - t16439 + t16440 + t16441 + t16443 + t16445 - t16449 - t16453 + t16456 + t16458 + t16463 - 16.0_f64 / 405.0_f64 * t12657;
    t18236
}

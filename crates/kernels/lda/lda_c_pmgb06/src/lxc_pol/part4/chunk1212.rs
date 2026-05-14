//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1212/1265 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1212<F: Float>(t12657: F, t16429: F, t16431: F, t16433: F, t16438: F, t16439: F, t16440: F, t16441: F, t16443: F, t16445: F, t16449: F, t16453: F, t16456: F, t16458: F, t16463: F, t12659: F, t12661: F, t16467: F, t16468: F, t16472: F, t16475: F, t16478: F, t16481: F, t16483: F, t16487: F, t16490: F, t16494: F, t16497: F, t16499: F, t16505: F) -> (F, F) {
    let t18236 = -t16429 - t16431 - t16433 + t16438 - t16439 + t16440 + t16441 + t16443 + t16445 - t16449 - t16453 + t16456 + t16458 + t16463 - 16.0 / 405.0 * t12657;
    let t18241 = -4.0 / 45.0 * t12659 + 8.0 / 135.0 * t12661 - t16467 - t16468 - t16472 - t16475 - t16478 - t16481 + t16483 + t16487 + t16490 + t16494 + t16497 - t16499 - t16505;
    (t18236, t18241)
}

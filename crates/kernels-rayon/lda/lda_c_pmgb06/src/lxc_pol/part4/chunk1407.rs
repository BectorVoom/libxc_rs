//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1407/1478 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1407(t12659: f64, t12661: f64, t16467: f64, t16468: f64, t16472: f64, t16475: f64, t16478: f64, t16481: f64, t16483: f64, t16487: f64, t16490: f64, t16494: f64, t16497: f64, t16499: f64, t16505: f64) -> f64 {
    let t18241 = -4.0_f64 / 45.0_f64 * t12659 + 8.0_f64 / 135.0_f64 * t12661 - t16467 - t16468 - t16472 - t16475 - t16478 - t16481 + t16483 + t16487 + t16490 + t16494 + t16497 - t16499 - t16505;
    t18241
}

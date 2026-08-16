//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 844/1478 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk844(t3531: f64, t3534: f64, t3569: f64, t3573: f64, t3583: f64, t3586: f64, t3597: f64, t5820: f64, t5821: f64, t5825: f64, t5826: f64, t3505: f64, t3513: f64, t3515: f64, t3517: f64, t3521: f64, t3523: f64, t3525: f64, t360: f64, t5805: f64, t5808: f64, t5810: f64, t5813: f64) -> f64 {
    let t5827 = -4.0_f64 / 9.0_f64 * t3531 + t3534 / 6.0_f64 - 0.97936_f64 * t3569 + 0.73452_f64 * t3573 + t5820 + t5821 - 1.95872_f64 * t3583 - t3586 / 2.0_f64 - 2.93808_f64 * t3597 - t5825 - t5826;
    let t5829 = t5805 + t5808 - t360 * t5810 / 2.0_f64 - 0.97936_f64 * t5813 - t3505 + t3513 - t3515 - t3517 - t3521 - t3523 + t3525 + t5827;
    t5829
}

//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 844/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk844(t5076: f64, t6545: f64, t6547: f64, t6549: f64, t6551: f64, t6553: f64, t6555: f64, t7637: f64, t7649: f64, t7653: f64, t7657: f64, t7779: f64) -> f64 {
    let t7791 = -0.21595_f64 * t7649 + 0.21595_f64 * t7653 - 0.07198333333333333_f64 * t6545 + 0.035991666666666665_f64 * t6547 + 0.023994444444444443_f64 * t6549 + 0.0044444444444444444_f64 * t6551 - 0.02666666666666667_f64 * t6553 + 0.013333333333333334_f64 * t6555 - 0.035991666666666665_f64 * t7657 - 0.03999074074074074_f64 * t7637 - 0.022222222222222223_f64 * t5076;
    let t7792 = t7779 + t7791;
    t7792
}

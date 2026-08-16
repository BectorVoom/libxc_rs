//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1263/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1263(t11: f64, t22653: f64, t503: f64, t331: f64, t7758: f64, t7764: f64, t11695: f64, t11709: f64, t11754: f64, t11781: f64, t15777: f64, t15779: f64, t15788: f64, t15798: f64, t15800: f64, t15820: f64, t17226: f64, t17234: f64, t22649: f64, t22651: f64, t25: f64, t538: f64, t9772: f64, t9813: f64) -> (f64, f64) {
    let t22655 = t11 * t503 * t22653;
    let t22660 = t331 * t7758;
    let t22662 = t331 * t7764;
    let t22664 = -0.007407407407407408_f64 * t11695 + 0.09597777777777777_f64 * t11709 - t11754 + 0.044444444444444446_f64 * t11781 + 0.019753086419753086_f64 * t9772 - 0.047988888888888886_f64 * t15777 + 0.09597777777777777_f64 * t15779 + 0.035991666666666665_f64 * t15788 + 0.03999074074074074_f64 * t15798 + 0.09597777777777777_f64 * t15800 - 0.022222222222222223_f64 * t17226 + 0.013333333333333334_f64 * t17234 + t9813 + 0.21595_f64 * t15820 + 0.0044444444444444444_f64 * t22649 + 0.0019753086419753087_f64 * t22651 - 0.035991666666666665_f64 * t22655 - 0.006666666666666667_f64 * t25 * t538 * t22653 - 0.008888888888888889_f64 * t22660 + 0.02666666666666667_f64 * t22662;
    (t22655, t22664)
}

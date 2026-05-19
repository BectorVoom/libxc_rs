//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1263/1365 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1263<F: Float>(t11: F, t22653: F, t503: F, t331: F, t7758: F, t7764: F, t11695: F, t11709: F, t11754: F, t11781: F, t15777: F, t15779: F, t15788: F, t15798: F, t15800: F, t15820: F, t17226: F, t17234: F, t22649: F, t22651: F, t25: F, t538: F, t9772: F, t9813: F) -> (F, F) {
    let t22655 = t11 * t503 * t22653;
    let t22660 = t331 * t7758;
    let t22662 = t331 * t7764;
    let t22664 = -F::cast_from(0.007407407407407408_f64) * t11695 + F::cast_from(0.09597777777777777_f64) * t11709 - t11754 + F::cast_from(0.044444444444444446_f64) * t11781 + F::cast_from(0.019753086419753086_f64) * t9772 - F::cast_from(0.047988888888888886_f64) * t15777 + F::cast_from(0.09597777777777777_f64) * t15779 + F::cast_from(0.035991666666666665_f64) * t15788 + F::cast_from(0.03999074074074074_f64) * t15798 + F::cast_from(0.09597777777777777_f64) * t15800 - F::cast_from(0.022222222222222223_f64) * t17226 + F::cast_from(0.013333333333333334_f64) * t17234 + t9813 + F::new(0.21595) * t15820 + F::cast_from(0.0044444444444444444_f64) * t22649 + F::cast_from(0.0019753086419753087_f64) * t22651 - F::cast_from(0.035991666666666665_f64) * t22655 - F::cast_from(0.006666666666666667_f64) * t25 * t538 * t22653 - F::cast_from(0.008888888888888889_f64) * t22660 + F::cast_from(0.02666666666666667_f64) * t22662;
    (t22655, t22664)
}

//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1232/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk1232<F: Float>(t2765: F, t5643: F, t159: F, t1904: F, t285: F, t39: F, t11510: F, t14480: F, t14485: F, t14488: F, t14491: F, t14500: F, t14503: F, t14505: F, t1568: F, t1808: F, t2793: F, t2811: F, t411: F, t4117: F, t5495: F, t5499: F, t5735: F, t5740: F, t5924: F, t6025: F, t777: F, t9138: F, t9166: F, t9169: F, t9174: F) -> F {
    let t14508 = t2765 * t5643;
    let t14515 = t39 * t1904 * t159 * t285;
    let t14516 = F::cast_from(0.004067943812504169_f64) * t14515;
    let t14517 = -F::cast_from(6.0_f64) * t777 * t9138 + F::cast_from(18.0_f64) * t5735 * t2793 + F::cast_from(18.0_f64) * t14480 * t2811 + F::cast_from(18.0_f64) * t6025 * t9169 + F::cast_from(18.0_f64) * t14485 * t9166 - F::cast_from(18.0_f64) * t14488 * t9174 + F::cast_from(18.0_f64) * t14491 * t5499 + F::cast_from(18.0_f64) * t1808 * t11510 * t411 + F::cast_from(18.0_f64) * t1808 * t5495 * t1568 + F::cast_from(18.0_f64) * t5924 * t14500 + F::cast_from(0.05987117005127304_f64) * t14503 + F::cast_from(18.0_f64) * t5924 * t14505 + F::cast_from(36.0_f64) * t5924 * t14508 + F::cast_from(9.0_f64) * t4117 * t5740 + t14516;
    t14517
}

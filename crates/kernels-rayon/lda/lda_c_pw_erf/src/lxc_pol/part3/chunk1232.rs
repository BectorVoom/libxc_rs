//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1232/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk1232(t2765: f64, t5643: f64, t159: f64, t1904: f64, t285: f64, t39: f64, t11510: f64, t14480: f64, t14485: f64, t14488: f64, t14491: f64, t14500: f64, t14503: f64, t14505: f64, t1568: f64, t1808: f64, t2793: f64, t2811: f64, t411: f64, t4117: f64, t5495: f64, t5499: f64, t5735: f64, t5740: f64, t5924: f64, t6025: f64, t777: f64, t9138: f64, t9166: f64, t9169: f64, t9174: f64) -> f64 {
    let t14508 = t2765 * t5643;
    let t14515 = t39 * t1904 * t159 * t285;
    let t14516 = 0.004067943812504169_f64 * t14515;
    let t14517 = -6.0_f64 * t777 * t9138 + 18.0_f64 * t5735 * t2793 + 18.0_f64 * t14480 * t2811 + 18.0_f64 * t6025 * t9169 + 18.0_f64 * t14485 * t9166 - 18.0_f64 * t14488 * t9174 + 18.0_f64 * t14491 * t5499 + 18.0_f64 * t1808 * t11510 * t411 + 18.0_f64 * t1808 * t5495 * t1568 + 18.0_f64 * t5924 * t14500 + 0.05987117005127304_f64 * t14503 + 18.0_f64 * t5924 * t14505 + 36.0_f64 * t5924 * t14508 + 9.0_f64 * t4117 * t5740 + t14516;
    t14517
}

//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 809/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk809(t128: f64, t5548: f64, t10: f64, t3268: f64, t3270: f64, t3274: f64, t3276: f64, t3213: f64, t3217: f64, t3220: f64, t3228: f64, t3231: f64, t3260: f64, t3264: f64, t426: f64, t5502: f64, t5505: f64, t5507: f64, t5511: f64, t5513: f64, t5517: f64, t5520: f64, t5523: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t5549 = t128 * t5548;
    let t5550 = t10 * t5549;
    let t5560 = 0.6495611111111111_f64 * t3268;
    let t5561 = 0.48717083333333333_f64 * t3270;
    let t5562 = 0.9743416666666667_f64 * t3274;
    let t5563 = 1.2991222222222223_f64 * t3276;
    let t5564 = -0.97936_f64 * t5502 - t5505 - 2.0_f64 / 9.0_f64 * t5507 + t5511 - 0.48968_f64 * t5513 + t5517 + t5520 + t5523 - t426 * t5550 / 2.0_f64 - 2.93808_f64 * t3213 - 1.95872_f64 * t3217 - t3220 / 2.0_f64 - 4.0_f64 / 9.0_f64 * t3228 + t3231 / 6.0_f64 - 0.97936_f64 * t3260 + 0.73452_f64 * t3264 + t5560 + t5561 - t5562 - t5563;
    (t5549, t5550, t5560, t5561, t5562, t5563, t5564)
}

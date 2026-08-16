//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 771/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk771(t127: f64, t3217: f64, t3228: f64, t3260: f64, t3280: f64, t3282: f64, t3284: f64, t3288: f64, t3290: f64, t411: f64, t5502: f64, t5507: f64, t5511: f64, t5513: f64, t5517: f64, t5523: f64, t7093: f64, t7096: f64, t7100: f64, t7101: f64, t7102: f64, t7108: f64) -> f64 {
    let t7109 = -1.95872_f64 * t5502 - t7093 - 4.0_f64 / 9.0_f64 * t5507 + t5511 - 0.97936_f64 * t5513 + t5517 + t7096 + t5523 - 0.97936_f64 * t3217 - 2.0_f64 / 9.0_f64 * t3228 - 0.48968_f64 * t3260 + t7100 - t7101 + t3280 - t3282 - t3284 - t3288 - t3290 + 5.87616_f64 * t127 * t7102 * t411 - t7108;
    t7109
}

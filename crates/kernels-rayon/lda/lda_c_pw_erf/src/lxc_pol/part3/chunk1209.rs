//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1209/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk1209(t14255: f64, t219: f64, t9408: f64, t13598: f64, t571: f64, t4753: f64, t4781: f64, t3416: f64, t14230: f64, t14233: f64, t14236: f64, t14239: f64, t14243: f64, t14246: f64, t14248: f64, t14250: f64, t14252: f64) -> (f64, f64, f64, f64, f64) {
    let t14256 = 8.0_f64 / 81.0_f64 * t14255;
    let t14257 = t9408 * t219;
    let t14260 = 352.0_f64 / 243.0_f64 * t571 * t14257 * t13598;
    let t14262 = 8.0_f64 / 15.0_f64 * t4753 * t4781;
    let t14264 = 8.0_f64 / 15.0_f64 * t3416 * t4781;
    let t14265 = t14230 + t14233 - t14236 - t14239 + t14243 + t14246 + t14248 + t14250 - t14252 - t14256 + t14260 + t14262 + t14264;
    (t14256, t14260, t14262, t14264, t14265)
}

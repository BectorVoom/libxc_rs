//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 692/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk692(t2334: f64, t408: f64, t2337: f64, t93: f64, t108: f64, t2268: f64, t2274: f64, t348: f64, t352: f64, t5992: f64, t6005: f64, t6164: f64, t6169: f64, t659: f64, t661: f64, t943: f64) -> (f64, f64) {
    let t6174 = t408 * t2334;
    let t6179 = t93 * t2337;
    let t6185 = (40.0_f64 / 27.0_f64 * t6164 * t348 + 80.0_f64 / 9.0_f64 * t2268 * t943 + 20.0_f64 / 9.0_f64 * t6169 * t348 + 4.0_f64 / 3.0_f64 * t659 * t5992 + 40.0_f64 / 27.0_f64 * t6174 * t352 - 80.0_f64 / 9.0_f64 * t2274 * t943 + 20.0_f64 / 9.0_f64 * t6179 * t352 + 4.0_f64 / 3.0_f64 * t661 * t6005) * t108;
    (t6174, t6185)
}

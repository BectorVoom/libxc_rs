//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 527/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk527(t668: f64, t858: f64, t739: f64, t92: f64, t34: f64, t659: f64, t743: f64, t93: f64, t661: f64, t108: f64, t348: f64, t352: f64, t462: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t2266 = t858 * t668;
    let t2268 = t92 * t739;
    let t2271 = t659 * t34;
    let t2274 = t93 * t743;
    let t2277 = t661 * t34;
    let t2281 = (20.0_f64 / 9.0_f64 * t2268 * t348 + 8.0_f64 / 3.0_f64 * t2271 * t462 + 20.0_f64 / 9.0_f64 * t2274 * t352 - 8.0_f64 / 3.0_f64 * t2277 * t462) * t108;
    (t2266, t2268, t2271, t2274, t2277, t2281)
}

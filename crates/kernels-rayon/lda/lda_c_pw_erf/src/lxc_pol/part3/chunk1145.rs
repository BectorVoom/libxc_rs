//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1145/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk1145(t10311: f64, t10315: f64, t10318: f64, t10320: f64, t10322: f64, t13396: f64, t13398: f64, t13400: f64, t13402: f64, t13403: f64, t13405: f64, t13407: f64, t13408: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t13409 = 8.0_f64 / 135.0_f64 * t10311;
    let t13410 = 8.0_f64 / 81.0_f64 * t10315;
    let t13411 = 16.0_f64 / 27.0_f64 * t10318;
    let t13412 = 8.0_f64 / 45.0_f64 * t10320;
    let t13413 = 8.0_f64 / 27.0_f64 * t10322;
    let t13414 = t13396 + t13398 + t13400 + t13402 + t13403 - t13405 + t13407 - t13408 - t13409 - t13410 - t13411 + t13412 + t13413;
    (t13409, t13410, t13411, t13412, t13413, t13414)
}

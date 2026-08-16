//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1215/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk1215(t1318: f64, t1401: f64, t1403: f64, t34: f64, t4892: f64, t3794: f64, t4946: f64, t14307: f64, t14311: f64, t14314: f64, t14317: f64, t14319: f64, t14321: f64, t14323: f64, t14327: f64, t14329: f64, t14331: f64, t14333: f64) -> (f64, f64, f64) {
    let t14338 = 8.0_f64 / 5.0_f64 * t1318 * t4892 * t1401 * t34 * t1403;
    let t14339 = t3794 * t4946;
    let t14340 = 16.0_f64 / 15.0_f64 * t14339;
    let t14341 = t14307 - t14311 + t14314 - t14317 + t14319 - t14321 - t14323 - t14327 - t14329 - t14331 + t14333 - t14338 - t14340;
    (t14338, t14340, t14341)
}

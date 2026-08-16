//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1093/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1093(t19645: f64, t19647: f64, t19650: f64, t10: f64, t128: f64, t20283: f64, t325: f64, t431: f64, t7930: f64, t415: f64, t7933: f64, t7924: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t20340 = 8.769075_f64 * t19645;
    let t20341 = 5.84605_f64 * t19647;
    let t20342 = 2.923025_f64 * t19650;
    let t20345 = t10 * t128 * t20283;
    let t20349 = t431 * t7930 * t325;
    let t20352 = t415 * t7933 * t325;
    let t20353 = 2.923025_f64 * t20352;
    let t20355 = t415 * t7924 * t325;
    (t20340, t20341, t20342, t20345, t20349, t20353, t20355)
}

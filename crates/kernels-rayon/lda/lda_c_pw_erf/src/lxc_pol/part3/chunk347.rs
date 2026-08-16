//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 347/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk347(t1234: f64, t185: f64, t473: f64, t56: f64, t174: f64, t177: f64, t325: f64, t506: f64, t521: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t1235 = t185 * t1234;
    let t1236 = 8.0_f64 / 45.0_f64 * t1235;
    let t1237 = t473 * t56;
    let t1239 = t174 * t1237 * t177;
    let t1240 = 0.047988888888888886_f64 * t1239;
    let t1241 = t325 * t506;
    let t1243 = t56 * t521;
    (t1235, t1236, t1237, t1239, t1240, t1241, t1243)
}

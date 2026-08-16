//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 329/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk329(t1217: f64, t265: f64, t665: f64, t668: f64, t514: f64, t543: f64, t185: f64, t473: f64, t56: f64, t174: f64, t177: f64, t325: f64, t506: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t1219 = 2.0_f64 / 135.0_f64 * t265 * t1217;
    let t1220 = t665 * t668;
    let t1234 = t514 * t543;
    let t1235 = t185 * t1234;
    let t1237 = t473 * t56;
    let t1239 = t174 * t1237 * t177;
    let t1240 = 0.047988888888888886_f64 * t1239;
    let t1241 = t325 * t506;
    (t1219, t1220, t1234, t1235, t1237, t1239, t1240, t1241)
}

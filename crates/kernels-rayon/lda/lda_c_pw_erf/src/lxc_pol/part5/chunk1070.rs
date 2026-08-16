//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1070/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1070(t8212: f64, t8216: f64, t19: f64, t729: f64, t734: f64, t8076: f64, t11362: f64, t8267: f64, t11349: f64, t11360: f64, t8221: f64, t8224: f64, t8238: f64, t8244: f64, t8248: f64, t8260: f64, t8263: f64, t8266: f64, t8271: f64, t8274: f64, t8277: f64) -> (f64, f64, f64, f64, f64) {
    let t20048 = 120.0_f64 * t8212;
    let t20049 = 12.0_f64 * t8216;
    let t20052 = t8076 * t729 * t19 * t734;
    let t20054 = 3.0_f64 * t11362;
    let t20055 = 0.021687161765563047_f64 * t8267;
    let t20056 = -t20048 - t20049 - t11349 - t8221 + t8224 + t8238 - t8244 - 0.41076328840066667_f64 * t20052 - t8248 + t8260 + t11360 + t20054 + t8263 - t8266 - t20055 + t8271 + t8274 - t8277;
    (t20048, t20049, t20054, t20055, t20056)
}

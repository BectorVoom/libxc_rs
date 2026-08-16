//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 598/1239 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk598(t144: f64, t3259: f64, t153: f64, t3092: f64, t3010: f64, t439: f64, t1420: f64, t1431: f64, t3212: f64, t3215: f64, t3219: f64, t3222: f64, t3225: f64, t3228: f64, t3230: f64, t3232: f64, t3234: f64, t3237: f64, t3241: f64, t3245: f64, t3253: f64, t3257: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t3260 = t3259 * t144;
    let t3261 = t153 * t3092;
    let t3262 = t3261 * t3010;
    let t3263 = t3260 * t3262;
    let t3265 = 8.0_f64 / 81.0_f64 * t439 * t3263;
    let t3267 = t1420 * t1431 / 15.0_f64;
    let t3268 = t3212 - t3215 + t3219 + t3222 - t3225 + t3228 + t3230 + t3232 + t3234 + t3237 + t3241 + t3245 + t3253 + t3257 + t3265 + t3267;
    (t3260, t3262, t3263, t3265, t3267, t3268)
}

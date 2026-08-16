//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1130/1239 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk1130(t1972: f64, t3169: f64, t1447: f64, t5319: f64, t1420: f64, t4772: f64, t1416: f64, t493: f64, t5312: f64, t432: f64, t4836: f64, t13327: f64, t13328: f64, t13421: f64, t13423: f64, t13425: f64, t13427: f64, t13429: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t13431 = 2.0_f64 / 15.0_f64 * t1972 * t3169;
    let t13432 = t1447 * t5319;
    let t13433 = 2.0_f64 / 15.0_f64 * t13432;
    let t13435 = 2.0_f64 / 15.0_f64 * t1420 * t4772;
    let t13438 = 2.0_f64 / 15.0_f64 * t493 * t5312 * t1416;
    let t13439 = t432 * t4836;
    let t13440 = t13439 / 45.0_f64;
    let t13441 = t13327 - t13328 + t13421 + t13423 + t13425 + t13427 - t13429 - t13431 + t13433 - t13435 - t13438 + t13440;
    (t13431, t13433, t13435, t13438, t13440, t13441)
}

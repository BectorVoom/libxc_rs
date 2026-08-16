//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1216/1478 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1216(t2010: f64, t5225: f64, t6189: f64, t2501: f64, t3177: f64, t1423: f64, t6413: f64, t6416: f64, t6419: f64, t1420: f64, t16000: f64, t16002: f64, t16006: f64, t16008: f64, t16011: f64, t16014: f64, t16018: f64, t16021: f64, t16023: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t16026 = 16.0_f64 / 45.0_f64 * t2010 * t5225 * t6189;
    let t16028 = 2.0_f64 / 45.0_f64 * t3177 * t2501;
    let t16029 = t1423 * t6413;
    let t16030 = 4.0_f64 / 135.0_f64 * t16029;
    let t16031 = t1423 * t6416;
    let t16032 = 8.0_f64 / 135.0_f64 * t16031;
    let t16033 = t1423 * t6419;
    let t16034 = 4.0_f64 / 81.0_f64 * t16033;
    let t16036 = 2.0_f64 / 45.0_f64 * t1420 * t6413;
    let t16037 = -t16000 + t16002 + t16006 + t16008 + t16011 + t16014 - t16018 + t16021 - t16023 - t16026 - t16028 - t16030 - t16032 + t16034 - t16036;
    (t16026, t16028, t16030, t16032, t16034, t16036, t16037)
}

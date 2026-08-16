//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 971/1239 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk971(t11211: f64, t11222: f64, t11225: f64, t11229: f64, t11236: f64, t11289: f64, t11470: f64, t11477: f64, t11485: f64, t11488: f64, t11491: f64, t1234: f64, t2209: f64, t2247: f64, t3588: f64, t5874: f64, t769: f64, t8428: f64) -> f64 {
    let t11493 = t11211 - t11222 + t11225 + t11229 - t11236 + t8428 - t11289 + 103.4553_f64 * t2247 * t11470 * t769 * t3588 + 20.69106_f64 * t11477 - 62.07318_f64 * t2247 * t5874 * t2209 * t1234 + 6.89702_f64 * t11485 - 10.34553_f64 * t11488 - 5.172765_f64 * t11491;
    t11493
}

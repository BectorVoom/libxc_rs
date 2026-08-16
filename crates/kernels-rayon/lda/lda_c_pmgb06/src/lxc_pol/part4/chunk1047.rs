//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1047/1478 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1047(t1786: f64, t1789: f64, t2368: f64, t409: f64, t328: f64, t5915: f64, t248: f64, t4515: f64, t686: f64, t2128: f64, t642: f64, t2136: f64) -> (f64, f64, f64, f64, f64) {
    let t10990 = t409 * t2368 * t1786 * t1789;
    let t10993 = t5915 * t328;
    let t11007 = t248 * t4515 * t686;
    let t11032 = 32.0_f64 * t2128 * t642;
    let t11058 = 32.0_f64 * t2136 * t642;
    (t10990, t10993, t11007, t11032, t11058)
}

//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 889/1239 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk889(t206: f64, t208: f64, t247: f64, t161: f64, t3004: f64, t512: f64, t3005: f64, t486: f64, t2943: f64, t495: f64, t224: f64, t3133: f64) -> (f64, f64, f64, f64, f64) {
    let t9348 = 0.19208479012345678_f64 * t206 * t247 * t208;
    let t9350 = t161 * t3004 * t512;
    let t9352 = t486 * t3005;
    let t9354 = t495 * t2943;
    let t9365 = t3133 * t224;
    (t9348, t9350, t9352, t9354, t9365)
}

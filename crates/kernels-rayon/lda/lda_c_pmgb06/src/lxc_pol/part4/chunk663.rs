//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 663/1478 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk663(t109: f64, t1234: f64, t55: f64, t3594: f64, t27: f64, t348: f64, t64: f64, t1243: f64, t3582: f64, t19: f64, t369: f64) -> (f64, f64, f64, f64, f64) {
    let t3596 = t55 * t109 * t1234;
    let t3597 = t3594 * t3596;
    let t3600 = t348 * t64 * t27;
    let t3601 = t3600 * t3596;
    let t3603 = t1243 * t3582;
    let t3615 = 1.0_f64 / t369 / t19;
    (t3597, t3600, t3601, t3603, t3615)
}

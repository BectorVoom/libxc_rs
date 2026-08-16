//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 625/1239 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk625(t1282: f64, t3588: f64, t35: f64, t27: f64, t365: f64, t109: f64, t1234: f64, t55: f64, t348: f64, t64: f64, t1243: f64, t3582: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t3589 = t1282 * t3588;
    let t3590 = t35 * t3589;
    let t3594 = t365 * t1282 * t27;
    let t3596 = t55 * t109 * t1234;
    let t3597 = t3594 * t3596;
    let t3600 = t348 * t64 * t27;
    let t3601 = t3600 * t3596;
    let t3602 = 2.923025_f64 * t3601;
    let t3603 = t1243 * t3582;
    (t3589, t3590, t3594, t3597, t3600, t3601, t3602, t3603)
}

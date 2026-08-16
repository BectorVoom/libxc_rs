//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 996/1239 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk996(t1499: f64, t2101: f64, t9317: f64, t3443: f64, t802: f64, t9330: f64, t9332: f64, t1988: f64, t3203: f64, t493: f64, t9338: f64, t9340: f64, t9342: f64, t9345: f64, t9348: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t11842 = t1499 * t2101 / 10.0_f64;
    let t11843 = 2.0_f64 / 15.0_f64 * t9317;
    let t11845 = t802 * t3443 / 30.0_f64;
    let t11846 = 4.0_f64 / 135.0_f64 * t9330;
    let t11847 = 2.0_f64 / 45.0_f64 * t9332;
    let t11853 = 2.0_f64 / 15.0_f64 * t493 * t1988 * t3203;
    let t11854 = -t11842 + t11843 - t11845 + t11846 - t11847 + 0.09973633333333333_f64 * t9338 + 0.299209_f64 * t9340 - 0.19947266666666666_f64 * t9342 - t9345 + t9348 + t11853;
    (t11842, t11843, t11845, t11846, t11847, t11853, t11854)
}

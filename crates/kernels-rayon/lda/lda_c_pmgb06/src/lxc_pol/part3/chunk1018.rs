//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1018/1239 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk1018(t12105: f64, t3076: f64, t802: f64, t1444: f64, t4757: f64, t132: f64, t1547: f64, t2107: f64, t9434: f64, t9441: f64, t9443: f64, t9450: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t12106 = 2.0_f64 / 15.0_f64 * t12105;
    let t12107 = t802 * t3076;
    let t12108 = t12107 / 15.0_f64;
    let t12110 = 2.0_f64 / 15.0_f64 * t1444 * t4757;
    let t12112 = t132 * t1547 * t2107;
    let t12113 = t12112 / 45.0_f64;
    let t12114 = t9434 / 15.0_f64;
    let t12115 = t9441 / 15.0_f64;
    let t12116 = 2.0_f64 / 15.0_f64 * t9443;
    let t12117 = t9450 / 15.0_f64;
    (t12106, t12108, t12110, t12113, t12114, t12115, t12116, t12117)
}

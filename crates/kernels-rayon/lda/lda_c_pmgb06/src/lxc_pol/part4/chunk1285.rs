//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1285/1478 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1285(t16884: f64, t13139: f64, t337: f64, t529: f64, t6560: f64, t12691: f64, t5068: f64, t13064: f64, t5138: f64, t13177: f64, t1083: f64, t2871: f64, t493: f64, t6516: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t16885 = 4.0_f64 / 45.0_f64 * t16884;
    let t16886 = 8.0_f64 / 135.0_f64 * t13139;
    let t16888 = t6560 * t529 * t337;
    let t16891 = 16.0_f64 / 45.0_f64 * t5068 * t12691 * t16888;
    let t16894 = 8.0_f64 / 27.0_f64 * t5138 * t13064 * t16888;
    let t16895 = 16.0_f64 / 1215.0_f64 * t13177;
    let t16899 = 2.0_f64 / 45.0_f64 * t493 * t2871 * t6516 * t1083;
    (t16885, t16886, t16891, t16894, t16895, t16899)
}

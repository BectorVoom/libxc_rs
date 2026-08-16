//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 733/1478 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk733(t12: f64, t1: f64, t15: f64, t1080: f64, t1083: f64, t1949: f64, t1952: f64, t247: f64, t395: f64, t4382: f64, t598: f64, t765: f64, t44: f64, t4697: f64, zeta_threshold: f64) -> f64 {
    let t13 = t12 <= zeta_threshold;
    let t4700 = t15 * t1;
    let t4710 = piecewise3(t13, 0.0_f64, 80.0_f64 / 27.0_f64 * t765 * t1080 - 160.0_f64 / 9.0_f64 * t4700 * t4382 + 40.0_f64 / 9.0_f64 * t1949 * t1083 - 16.0_f64 / 3.0_f64 * t598 * t395 + 16.0_f64 * t1952 * t247);
    let t4713 = (t4697 / 2.0_f64 + t4710 / 2.0_f64) * t44;
    t4713
}

//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 725/1239 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk725(t4801: f64, t529: f64, t166: f64, t161: f64, t1887: f64, t436: f64, t1928: f64, t432: f64, t1873: f64, t435: f64, t132: f64, t1517: f64, t802: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t4802 = t4801 * t529;
    let t4803 = t166 * t4802;
    let t4805 = t161 * t4803 / 15.0_f64;
    let t4807 = 2.0_f64 / 45.0_f64 * t1887 * t436;
    let t4809 = 2.0_f64 / 45.0_f64 * t432 * t1928;
    let t4810 = t435 * t1873;
    let t4812 = 2.0_f64 / 45.0_f64 * t132 * t4810;
    let t4814 = 2.0_f64 / 45.0_f64 * t802 * t1517;
    (t4802, t4803, t4805, t4807, t4809, t4810, t4812, t4814)
}

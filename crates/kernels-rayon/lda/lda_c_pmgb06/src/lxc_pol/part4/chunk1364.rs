//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1364/1478 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1364(t1887: f64, t1928: f64, t4810: f64, t802: f64, t14024: f64, t14068: f64, t1554: f64, t161: f64, t2624: f64, t1512: f64, t2650: f64, t132: f64, t1547: f64, t2630: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t17919 = t1887 * t1928;
    let t17920 = 4.0_f64 / 45.0_f64 * t17919;
    let t17921 = t802 * t4810;
    let t17922 = 4.0_f64 / 45.0_f64 * t17921;
    let t17923 = 4.0_f64 / 45.0_f64 * t14024;
    let t17924 = 2.0_f64 / 45.0_f64 * t14068;
    let t17926 = t161 * t1554 * t2624;
    let t17927 = t17926 / 135.0_f64;
    let t17929 = t1512 * t2650 / 30.0_f64;
    let t17931 = t132 * t1547 * t2630;
    (t17920, t17922, t17923, t17924, t17927, t17929, t17931)
}

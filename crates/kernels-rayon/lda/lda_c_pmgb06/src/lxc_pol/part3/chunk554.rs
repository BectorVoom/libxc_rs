//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 554/1239 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk554(t1179: f64, t139: f64, t138: f64, t163: f64, t508: f64, t947: f64, t1478: f64, t350: f64, t1482: f64, t1486: f64, t1461: f64, t1463: f64, t158: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t2897 = t1179 * t139;
    let t2899 = t138 * t2897 * t163;
    let t2900 = 0.005877407407407408_f64 * t2899;
    let t2901 = t947 * t508;
    let t2903 = t350 * t1478;
    let t2905 = t350 * t1482;
    let t2907 = t350 * t1486;
    let t2909 = t139 * t1461;
    let t2911 = 1.0_f64 / t1463 / t158;
    (t2897, t2899, t2900, t2901, t2903, t2905, t2907, t2909, t2911)
}

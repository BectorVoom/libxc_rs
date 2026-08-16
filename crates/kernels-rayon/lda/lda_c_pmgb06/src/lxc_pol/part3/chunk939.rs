//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 939/1239 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk939(t1152: f64, t1200: f64, t123: f64, t10793: f64, t199: f64, t2822: f64, t566: f64, t4209: f64, t722: f64, t101: f64, t4329: f64, t754: f64, t757: f64) -> (f64, f64, f64, f64, f64) {
    let t10940 = t123 * t1152 * t1200;
    let t10943 = t123 * t10793 * t199;
    let t10946 = t123 * t2822 * t566;
    let t10949 = t123 * t722 * t4209;
    let t10960 = t101 * t4329 * t754 * t757;
    (t10940, t10943, t10946, t10949, t10960)
}

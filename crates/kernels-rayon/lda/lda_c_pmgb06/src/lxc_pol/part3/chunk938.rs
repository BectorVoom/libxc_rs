//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 938/1239 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk938(t642: f64, t701: f64, t123: f64, t199: f64, t2803: f64, t315: f64, t4259: f64, t566: f64, t1156: f64, t1200: f64, t10797: f64, t2833: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t10905 = t642 * t701;
    let t10925 = t123 * t315 * t2803 * t199;
    let t10928 = t123 * t4259 * t566;
    let t10931 = t123 * t1156 * t1200;
    let t10934 = t123 * t10797 * t199;
    let t10937 = t123 * t2833 * t566;
    (t10905, t10925, t10928, t10931, t10934, t10937)
}

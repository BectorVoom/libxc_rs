//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 378/1239 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk378(t1409: f64, t83: f64, t188: f64, t12: f64, t158: f64) -> (f64, f64, f64, f64) {
    let t1410 = t83 * t1409;
    let t1412 = 4.0_f64 / 3.0_f64 * t1410 * t188;
    let t1413 = t158 * t12;
    let t1414 = 1.0_f64 / t1413;
    (t1410, t1412, t1413, t1414)
}

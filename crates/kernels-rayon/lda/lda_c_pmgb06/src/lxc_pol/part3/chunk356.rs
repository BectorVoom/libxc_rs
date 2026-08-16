//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 356/1239 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk356(t1289: f64, t1291: f64, t1296: f64, t1297: f64, t1309: f64, t378: f64, t384: f64, t74: f64, t387: f64) -> (f64, f64) {
    let t1311 = t1289 * t74 - 2.0_f64 * t1291 * t384 + 2.0_f64 * t1296 * t1297 - t378 * t1309;
    let t1312 = t1311 * t387;
    (t1311, t1312)
}

//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1156/1267 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk1156(t2497: f64, t5305: f64, t1972: f64, t6387: f64, t6391: f64, t6268: f64, t6395: f64, t17734: f64, t17736: f64, t17738: f64, t10321: f64, t493: f64, t6113: f64, t6119: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t20888 = 2.0_f64 / 15.0_f64 * t5305 * t2497;
    let t20890 = 2.0_f64 / 15.0_f64 * t1972 * t6387;
    let t20892 = 2.0_f64 / 15.0_f64 * t1972 * t6391;
    let t20894 = 4.0_f64 / 15.0_f64 * t6268 * t6395;
    let t20895 = 8.0_f64 / 45.0_f64 * t17734;
    let t20896 = 8.0_f64 / 45.0_f64 * t17736;
    let t20897 = 4.0_f64 / 27.0_f64 * t17738;
    let t20898 = 8.0_f64 / 1215.0_f64 * t10321;
    let t20901 = t493 * t6119 * t6113 / 5.0_f64;
    (t20888, t20890, t20892, t20894, t20895, t20896, t20897, t20898, t20901)
}

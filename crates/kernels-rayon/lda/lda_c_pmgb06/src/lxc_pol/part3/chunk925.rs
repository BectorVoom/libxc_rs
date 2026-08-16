//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 925/1239 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk925(t1139: f64, t1183: f64, t301: f64, t1100: f64, t83: f64, t113: f64, t2778: f64, t413: f64, t398: f64, t642: f64, t1234: f64, t384: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t10635 = t1139 * t1183 * t301;
    let t10637 = t1100 * t83;
    let t10640 = 0.03831185177913979_f64 * t10637 * t113 * t301;
    let t10643 = 0.026861343269868797_f64 * t2778 * t413 * t301;
    let t10644 = t642 * t398;
    let t10646 = t10644 * t113 * t301;
    let t10648 = t384 * t1234;
    (t10635, t10637, t10640, t10643, t10644, t10646, t10648)
}

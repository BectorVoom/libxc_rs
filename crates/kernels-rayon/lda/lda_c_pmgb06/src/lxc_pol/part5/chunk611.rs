//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 611/1267 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk611(t3922: f64, t764: f64, t1: f64, t1079: f64, t2160: f64, t638: f64, t1105: f64, t898: f64, t1101: f64, t1065: f64, t897: f64, t248: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t4500 = t3922 * t764;
    let t4503 = t1079 * t1;
    let t4518 = t638 * t2160;
    let t4520 = t1105 * t898;
    let t4522 = t1101 * t898;
    let t4524 = t897 * t1065;
    let t4525 = t248 * t4524;
    (t4500, t4503, t4518, t4520, t4522, t4524, t4525)
}

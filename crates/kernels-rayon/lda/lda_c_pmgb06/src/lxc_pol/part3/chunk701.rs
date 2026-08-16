//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 701/1239 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk701(t285: f64, t4515: f64, t2160: f64, t638: f64, t1105: f64, t898: f64, t1101: f64, t1065: f64, t897: f64, t248: f64, t1108: f64, t2142: f64, t27: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t4516 = t4515 * t285;
    let t4518 = t638 * t2160;
    let t4520 = t1105 * t898;
    let t4522 = t1101 * t898;
    let t4524 = t897 * t1065;
    let t4525 = t248 * t4524;
    let t4527 = t1108 * t898;
    let t4529 = t2142 * t27;
    (t4516, t4518, t4520, t4522, t4524, t4525, t4527, t4529)
}

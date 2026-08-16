//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 537/1239 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk537(t315: f64, t52: f64, t934: f64, t1147: f64, t117: f64, t123: f64, t191: f64, t642: f64, t83: f64) -> (f64, f64, f64) {
    let t2771 = t934 * t315 * t52;
    let t2777 = 0.0878110494085338_f64 * t123 * t1147 * t191 * t117;
    let t2778 = t642 * t83;
    (t2771, t2777, t2778)
}

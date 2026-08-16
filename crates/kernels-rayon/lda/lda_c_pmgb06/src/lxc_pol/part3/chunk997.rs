//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 997/1239 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk997(t1992: f64, t3459: f64, t493: f64, t851: f64, t9636: f64, t2007: f64, t3213: f64, t131: f64, t1767: f64, t129: f64, t2012: f64, t10318: f64, t806: f64) -> (f64, f64, f64, f64, f64) {
    let t11859 = 4.0_f64 / 5.0_f64 * t493 * t1992 * t9636 * t851 * t3459;
    let t11860 = t3213 * t2007;
    let t11861 = 2.0_f64 / 135.0_f64 * t11860;
    let t11862 = t131 * t1767;
    let t11864 = t129 * t11862 * t2012;
    let t11865 = 32.0_f64 / 135.0_f64 * t11864;
    let t11866 = t10318 * t806;
    (t11859, t11861, t11862, t11865, t11866)
}

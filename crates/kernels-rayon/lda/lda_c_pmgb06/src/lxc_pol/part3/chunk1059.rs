//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1059/1239 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk1059(t1083: f64, t4865: f64, t1919: f64, t1981: f64, t10230: f64, t176: f64, t2912: f64, t764: f64, t9509: f64, t493: f64, t1: f64, t1080: f64, t2911: f64) -> (f64, f64, f64, f64, f64) {
    let t12588 = t4865 * t1083;
    let t12591 = 2.0_f64 / 9.0_f64 * t1981 * t1919 * t12588;
    let t12592 = t10230 * t176;
    let t12594 = t9509 * t764 * t2912;
    let t12597 = 88.0_f64 / 243.0_f64 * t493 * t12592 * t12594;
    let t12599 = t2911 * t1 * t1080;
    (t12588, t12591, t12594, t12597, t12599)
}

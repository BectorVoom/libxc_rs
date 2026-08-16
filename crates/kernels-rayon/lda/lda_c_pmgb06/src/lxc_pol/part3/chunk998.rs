//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 998/1239 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk998(t11866: f64, t436: f64, t4754: f64, t1559: f64, t439: f64, t4779: f64, t2002: f64, t3186: f64, t1925: f64, t3226: f64, t1600: f64, t1988: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t11867 = 2.0_f64 / 135.0_f64 * t11866;
    let t11868 = t4754 * t436;
    let t11869 = t11868 / 15.0_f64;
    let t11872 = 2.0_f64 / 15.0_f64 * t439 * t4779 * t1559;
    let t11874 = 2.0_f64 / 15.0_f64 * t2002 * t3186;
    let t11875 = t3226 * t1925;
    let t11876 = 4.0_f64 / 45.0_f64 * t11875;
    let t11877 = t1988 * t1600;
    (t11867, t11869, t11872, t11874, t11876, t11877)
}

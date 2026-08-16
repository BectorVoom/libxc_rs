//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 299/1478 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk299(t667: f64, t993: f64, t942: f64, t945: f64, t947: f64, t951: f64, t953: f64, t955: f64) -> (f64, f64) {
    let t994 = t993 * t667;
    let t1003 = -0.7843833333333333_f64 * t942 + 1.5687666666666666_f64 * t945 + 0.6886333333333333_f64 * t947 + 0.14025833333333335_f64 * t951 + 0.2805166666666667_f64 * t953 + 0.17365833333333333_f64 * t955;
    (t994, t1003)
}

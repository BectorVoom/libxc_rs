//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 869/1239 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk869(t675: f64, t682: f64, t696: f64, t8719: f64, t3711: f64, t971: f64, t1066: f64, t1101: f64, t3934: f64, t643: f64, t1026: f64, t1035: f64, t1041: f64) -> (f64, f64, f64, f64, f64) {
    let t8723 = 0.5848223622634646_f64 * t696 * t675 * t8719 * t682;
    let t8724 = t971 * t3711;
    let t8727 = 120.0_f64 * t1101 * t1066;
    let t8729 = t643 * t3934;
    let t8733 = 36.0_f64 * t1041 * t1026 * t1035;
    (t8723, t8724, t8727, t8729, t8733)
}

//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 618/1239 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk618(t28: f64, t64: f64, t247: f64, t3509: f64, t1830: f64, t366: f64, t349: f64, t1179: f64, t54: f64, t55: f64, t56: f64, t1272: f64, t2060: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t3510 = t64 * t28;
    let t3513 = 0.3264533333333333_f64 * t3509 * t3510 * t247;
    let t3515 = 0.7617244444444444_f64 * t366 * t1830;
    let t3517 = 1.5156425925925925_f64 * t349 * t1830;
    let t3521 = 7.0_f64 / 27.0_f64 * t54 * t55 * t1179 * t56;
    let t3523 = 0.6529066666666666_f64 * t1272 * t2060;
    (t3510, t3513, t3515, t3517, t3521, t3523)
}

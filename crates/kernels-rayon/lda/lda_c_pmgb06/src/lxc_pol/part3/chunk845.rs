//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 845/1239 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk845(t3501: f64, t56: f64, t247: f64, t28: f64, t342: f64, t3509: f64, t370: f64, t366: f64, t4641: f64, t349: f64, t1767: f64, t54: f64, t55: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t8276 = t3501 * t56;
    let t8278 = t28 * t247 * t342;
    let t8279 = t8276 * t8278;
    let t8281 = t3509 * t370;
    let t8282 = t8281 * t8278;
    let t8285 = 2.5390814814814813_f64 * t366 * t4641;
    let t8287 = 5.052141975308642_f64 * t349 * t4641;
    let t8291 = 70.0_f64 / 81.0_f64 * t54 * t55 * t1767 * t56;
    (t8276, t8279, t8281, t8282, t8285, t8287, t8291)
}

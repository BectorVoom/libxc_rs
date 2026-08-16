//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 909/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk909(t1390: f64, t1449: f64, t3762: f64, t581: f64, t3675: f64, t522: f64, t1351: f64, t212: f64, t22: f64, t1350: f64, t155: f64, t213: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t9304 = t1449 * t1390;
    let t9313 = t3762 * t581;
    let t9351 = t522 * t3675;
    let t9408 = t22 / t212 / t1351;
    let t9409 = t1350 * t1350;
    let t9410 = 1.0_f64 / t9409;
    let t9432 = t155 * t213;
    (t9304, t9313, t9351, t9408, t9410, t9432)
}

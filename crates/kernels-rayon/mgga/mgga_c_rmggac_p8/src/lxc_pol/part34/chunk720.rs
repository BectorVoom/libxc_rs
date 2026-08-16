//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 720/1097 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk720(t13957: f64, t36292: f64, t739: f64, t14012: f64, t14371: f64, t1341: f64, t638: f64, t669: f64, t7310: f64, t1302: f64, t14148: f64, t14149: f64, t20: f64, t7351: f64) -> (f64, f64, f64, f64) {
    let t70225 = t739 * t36292 * t13957;
    let t70229 = t14371 * t14012;
    let t70237 = t638 * t7310 * t669 * t1341;
    let t70271 = t14148 * t7351 * t14149 * t1302 * t20;
    (t70225, t70229, t70237, t70271)
}

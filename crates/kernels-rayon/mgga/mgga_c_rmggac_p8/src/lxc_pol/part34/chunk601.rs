//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 601/1097 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk601(t2411: f64, t3148: f64, t3151: f64, t14011: f64, t560: f64, t3120: f64, t572: f64, t3112: f64, t597: f64, t201: f64, t14022: f64, t14027: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t15331 = t2411 * t3148 * t3151;
    let t15333 = t14011 * t560;
    let t15334 = t3120 * t15333;
    let t15336 = t14011 * t572;
    let t15337 = t3120 * t15336;
    let t15339 = t3112 * t597;
    let t15340 = t15339 * t201;
    let t15342 = t15340 * t14022 * t14027;
    (t15331, t15333, t15334, t15336, t15337, t15339, t15340, t15342)
}

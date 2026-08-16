//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 1031/1127 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk1031(t8832: f64, t8837: f64, t8844: f64, t8846: f64, t8852: f64, t8856: f64, t8860: f64, t8864: f64, t8872: f64, t9597: f64, t9599: f64, t10376: f64, t37183: f64, t7780: f64, t8200: f64) -> (f64, f64, f64, f64, f64) {
    let t42528 = 0.638468998399467591e-4_f64 * t8832;
    let t42529 = 0.638468998399467591e-4_f64 * t8837;
    let t42530 = 0.212822999466489197e-4_f64 * t8844;
    let t42531 = 0.212822999466489197e-4_f64 * t8846;
    let t42534 = 0.60975299583150056624e-3_f64 * t8852;
    let t42535 = 0.60975299583150056624e-3_f64 * t8856;
    let t42536 = 0.60975299583150056624e-3_f64 * t8860;
    let t42537 = 0.60975299583150056624e-3_f64 * t8864;
    let t42539 = 0.17961362552795712846e0_f64 * t8872;
    let t42540 = 2.0_f64 * t9597;
    let t42541 = 2.0_f64 * t9599;
    let t42542 = t42534 + t42535 + t42536 + t42537 + t10376 - 0.31931311204970156171e0_f64 * t7780 + t42539 + t42540 + t8200 + t37183 + t42541;
    (t42528, t42529, t42530, t42531, t42542)
}

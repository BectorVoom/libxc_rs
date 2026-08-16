//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 1226/1276 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk1226(t40041: f64, t40044: f64, t40047: f64, t40050: f64, t40053: f64, t38036: f64, t40024: f64, t40027: f64, t40029: f64, t40031: f64, t40035: f64, t40038: f64) -> f64 {
    let t41668 = 0.93149212406257582492e-1_f64 * t40041;
    let t41669 = 0.27944763721877274748e0_f64 * t40044;
    let t41670 = 0.93149212406257582492e-1_f64 * t40047;
    let t41671 = 0.27944763721877274748e0_f64 * t40050;
    let t41672 = 0.93149212406257582492e-1_f64 * t40053;
    let t41673 = -0.17336443480108537126e0_f64 * t40024 - 0.86682217400542685632e-1_f64 * t40027 - 0.17336443480108537126e0_f64 * t40029 - 0.86682217400542685632e-1_f64 * t40031 - 0.87327386630866483588e-2_f64 * t40035 + 0.27944763721877274748e0_f64 * t38036 + 0.17336443480108537126e0_f64 * t40038 + t41668 + t41669 + t41670 + t41671 - t41672;
    t41673
}

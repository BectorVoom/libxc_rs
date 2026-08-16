//! MGGA_C_RMGGAC lxc pol — lxc_pol part 17 (v4rho3sigma_8) CSE chunk 1001/1111 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part17_v4rho3sigma_8_chunk1001(t41404: f64, t46106: f64, t40999: f64, t46109: f64, t35960: f64, t649: f64, t6530: f64, t25854: f64, t27055: f64, t27176: f64, t41120: f64, t41129: f64, t46455: f64, t46457: f64, t46459: f64, t46462: f64, t46465: f64, t46468: f64, t6444: f64, t9840: f64) -> f64 {
    let t46471 = t41404 * t46106;
    let t46473 = t40999 * t46109;
    let t46476 = t35960 * t649 * t6530;
    let t46478 = 0.11974241701863808564e0_f64 * t6444 * t9840 + 0.72732431077987577941e-1_f64 * t46455 - 0.21819729323396273382e0_f64 * t46457 - 0.54549323308490683456e-1_f64 * t46459 + 0.47896966807455234255e0_f64 * t41120 + 0.71845450211182851384e0_f64 * t25854 * t46462 - 0.95793933614910468512e0_f64 * t27176 * t46465 - 0.71845450211182851384e0_f64 * t27055 * t46468 - t41129 + 0.16364796992547205037e0_f64 * t46471 - 0.40911992481368012592e0_f64 * t46473 - 0.81823984962736025184e-1_f64 * t46476;
    t46478
}

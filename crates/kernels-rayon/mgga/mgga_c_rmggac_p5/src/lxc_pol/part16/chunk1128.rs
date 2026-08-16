//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 1128/1158 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk1128(t25854: f64, t27176: f64, t35922: f64, t35926: f64, t41120: f64, t44143: f64, t44145: f64, t46439: f64, t46442: f64, t46445: f64, t46455: f64, t46457: f64, t46459: f64, t48281: f64, t48284: f64) -> f64 {
    let t49365 = 0.20455996240684006298e-1_f64 * t46439 + 0.35922725105591425692e0_f64 * t46442 - 0.71845450211182851384e0_f64 * t46445 + 0.71845450211182851384e0_f64 * t25854 * t48281 - 0.95793933614910468512e0_f64 * t27176 * t48284 + 0.66671395154821946452e-1_f64 * t35922 + 0.26668558061928778581e0_f64 * t35926 + t44143 + 0.14546486215597515589e0_f64 * t46455 - 0.43639458646792546768e0_f64 * t46457 - 0.10909864661698136692e0_f64 * t46459 + 0.9579393361491046851e0_f64 * t41120 - t44145;
    t49365
}

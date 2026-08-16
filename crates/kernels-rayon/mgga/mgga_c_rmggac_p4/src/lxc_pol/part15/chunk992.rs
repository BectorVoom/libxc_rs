//! MGGA_C_RMGGAC lxc pol — lxc_pol part 15 (v4rho3sigma_6) CSE chunk 992/1110 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part15_v4rho3sigma_6_chunk992(t35960: f64, t649: f64, t6583: f64, t41400: f64, t6586: f64, t40932: f64, t6558: f64, t118: f64, t25820: f64, t25854: f64, t25877: f64, t40967: f64, t40970: f64, t46324: f64, t46327: f64, t46329: f64, t46331: f64, t46333: f64, t46336: f64, t46339: f64) -> f64 {
    let t46343 = t35960 * t649 * t6583;
    let t46346 = t41400 * t649 * t6586;
    let t46349 = t40932 * t649 * t6558;
    let t46352 = -0.79828278012425390428e-1_f64 * t118 * t46324 - 0.20455996240684006296e0_f64 * t46327 + 0.40911992481368012592e0_f64 * t46329 + 0.6818665413561335432e-1_f64 * t46331 - 0.71845450211182851384e0_f64 * t25820 * t46333 + 0.14369090042236570277e1_f64 * t25877 * t46336 + 0.71845450211182851384e0_f64 * t25854 * t46339 - 0.81823984962736025184e-1_f64 * t46343 + 0.13637330827122670864e0_f64 * t46346 + 0.54549323308490683456e-1_f64 * t46349 + t40967 - 0.54549323308490683458e-1_f64 * t40970;
    t46352
}

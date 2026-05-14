//! MGGA_C_RMGGAC lxc pol — lxc_pol part 15 (v4rho3sigma_6) CSE chunk 878/963 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part15_v4rho3sigma_6_chunk878<F: Float>(t35960: F, t649: F, t6530: F, t25854: F, t27055: F, t27176: F, t41120: F, t41129: F, t46455: F, t46457: F, t46459: F, t46462: F, t46465: F, t46468: F, t46471: F, t46473: F, t6444: F, t9840: F) -> (F,) {
    let t46476 = t35960 * t649 * t6530;
    let t46478 = 0.11974241701863808564e0 * t6444 * t9840 + 0.72732431077987577941e-1 * t46455 - 0.21819729323396273382e0 * t46457 - 0.54549323308490683456e-1 * t46459 + 0.47896966807455234255e0 * t41120 + 0.71845450211182851384e0 * t25854 * t46462 - 0.95793933614910468512e0 * t27176 * t46465 - 0.71845450211182851384e0 * t27055 * t46468 - t41129 + 0.16364796992547205037e0 * t46471 - 0.40911992481368012592e0 * t46473 - 0.81823984962736025184e-1 * t46476;
    (t46478,)
}

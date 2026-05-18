//! MGGA_C_RMGGAC lxc pol — lxc_pol part 15 (v4rho3sigma_6) CSE chunk 992/1110 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part15_v4rho3sigma_6_chunk992<F: Float>(t35960: F, t649: F, t6583: F, t41400: F, t6586: F, t40932: F, t6558: F, t118: F, t25820: F, t25854: F, t25877: F, t40967: F, t40970: F, t46324: F, t46327: F, t46329: F, t46331: F, t46333: F, t46336: F, t46339: F) -> F {
    let t46343 = t35960 * t649 * t6583;
    let t46346 = t41400 * t649 * t6586;
    let t46349 = t40932 * t649 * t6558;
    let t46352 = -F::new(0.79828278012425390428e-1) * t118 * t46324 - F::new(0.20455996240684006296e0) * t46327 + F::new(0.40911992481368012592e0) * t46329 + F::new(0.6818665413561335432e-1) * t46331 - F::new(0.71845450211182851384e0) * t25820 * t46333 + F::new(0.14369090042236570277e1) * t25877 * t46336 + F::new(0.71845450211182851384e0) * t25854 * t46339 - F::new(0.81823984962736025184e-1) * t46343 + F::new(0.13637330827122670864e0) * t46346 + F::new(0.54549323308490683456e-1) * t46349 + t40967 - F::new(0.54549323308490683458e-1) * t40970;
    t46352
}

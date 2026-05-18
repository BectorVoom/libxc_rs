//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 1128/1158 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk1128<F: Float>(t25854: F, t27176: F, t35922: F, t35926: F, t41120: F, t44143: F, t44145: F, t46439: F, t46442: F, t46445: F, t46455: F, t46457: F, t46459: F, t48281: F, t48284: F) -> F {
    let t49365 = F::new(0.20455996240684006298e-1) * t46439 + F::new(0.35922725105591425692e0) * t46442 - F::new(0.71845450211182851384e0) * t46445 + F::new(0.71845450211182851384e0) * t25854 * t48281 - F::new(0.95793933614910468512e0) * t27176 * t48284 + F::new(0.66671395154821946452e-1) * t35922 + F::new(0.26668558061928778581e0) * t35926 + t44143 + F::new(0.14546486215597515589e0) * t46455 - F::new(0.43639458646792546768e0) * t46457 - F::new(0.10909864661698136692e0) * t46459 + F::new(0.9579393361491046851e0) * t41120 - t44145;
    t49365
}

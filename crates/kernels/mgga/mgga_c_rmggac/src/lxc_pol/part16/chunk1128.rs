//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 1128/1158 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk1128<F: Float>(t25854: F, t27176: F, t35922: F, t35926: F, t41120: F, t44143: F, t44145: F, t46439: F, t46442: F, t46445: F, t46455: F, t46457: F, t46459: F, t48281: F, t48284: F) -> F {
    let t49365 = F::cast_from(0.20455996240684006298e-1_f64) * t46439 + F::cast_from(0.35922725105591425692e0_f64) * t46442 - F::cast_from(0.71845450211182851384e0_f64) * t46445 + F::cast_from(0.71845450211182851384e0_f64) * t25854 * t48281 - F::cast_from(0.95793933614910468512e0_f64) * t27176 * t48284 + F::cast_from(0.66671395154821946452e-1_f64) * t35922 + F::cast_from(0.26668558061928778581e0_f64) * t35926 + t44143 + F::cast_from(0.14546486215597515589e0_f64) * t46455 - F::cast_from(0.43639458646792546768e0_f64) * t46457 - F::cast_from(0.10909864661698136692e0_f64) * t46459 + F::cast_from(0.9579393361491046851e0_f64) * t41120 - t44145;
    t49365
}

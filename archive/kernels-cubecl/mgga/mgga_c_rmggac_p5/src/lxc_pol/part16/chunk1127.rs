//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 1127/1158 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk1127<F: Float>(t46389: F, t46392: F, t46395: F, t46398: F, t46409: F, t46413: F, t46417: F, t46421: F, t46425: F, t46429: F, t46432: F, t46435: F) -> F {
    let t49351 = F::cast_from(0.23948483403727617128e0_f64) * t46389 + F::cast_from(0.14369090042236570277e1_f64) * t46392 + F::cast_from(0.35922725105591425692e0_f64) * t46395 - F::cast_from(0.35922725105591425692e0_f64) * t46398 + F::cast_from(0.8182398496273602519e-1_f64) * t46409 - F::cast_from(0.13637330827122670865e0_f64) * t46413 - F::cast_from(0.2727466165424534173e-1_f64) * t46417 + F::cast_from(0.20455996240684006298e-1_f64) * t46421 - F::cast_from(0.2727466165424534173e-1_f64) * t46425 - F::cast_from(0.13637330827122670865e-1_f64) * t46429 - F::cast_from(0.40911992481368012596e-1_f64) * t46432 + F::cast_from(0.81823984962736025192e-1_f64) * t46435;
    t49351
}

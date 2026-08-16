//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 1127/1158 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk1127(t46389: f64, t46392: f64, t46395: f64, t46398: f64, t46409: f64, t46413: f64, t46417: f64, t46421: f64, t46425: f64, t46429: f64, t46432: f64, t46435: f64) -> f64 {
    let t49351 = 0.23948483403727617128e0_f64 * t46389 + 0.14369090042236570277e1_f64 * t46392 + 0.35922725105591425692e0_f64 * t46395 - 0.35922725105591425692e0_f64 * t46398 + 0.8182398496273602519e-1_f64 * t46409 - 0.13637330827122670865e0_f64 * t46413 - 0.2727466165424534173e-1_f64 * t46417 + 0.20455996240684006298e-1_f64 * t46421 - 0.2727466165424534173e-1_f64 * t46425 - 0.13637330827122670865e-1_f64 * t46429 - 0.40911992481368012596e-1_f64 * t46432 + 0.81823984962736025192e-1_f64 * t46435;
    t49351
}

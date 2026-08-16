//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 1134/1158 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk1134(t1763: f64, t698: f64, t2447: f64, t570: f64, t27055: f64, t321: f64, t333: f64, t352: f64, t44187: f64, t44239: f64, t46662: f64, t46664: f64, t46669: f64, t46671: f64, t46673: f64, t46675: f64, t46677: f64, t46686: f64, t4669: f64, t49411: f64, t5148: f64, t558: f64, t8940: f64) -> (f64, f64, f64) {
    let t49480 = t698 * t1763;
    let t49493 = t2447 * t570;
    let t49501 = -0.35922725105591425692e0_f64 * t46662 - 0.23948483403727617128e0_f64 * t5148 * t44239 * t570 - 0.35922725105591425692e0_f64 * t27055 * t49480 * t333 - 0.35922725105591425692e0_f64 * t46664 - 0.11974241701863808564e0_f64 * t46669 - 0.35922725105591425692e0_f64 * t46671 - 0.35922725105591425692e0_f64 * t46673 - 0.11974241701863808564e0_f64 * t5148 * t49411 * t321 + 0.17961362552795712846e0_f64 * t46675 + 0.71845450211182851384e0_f64 * t46677 + 0.23948483403727617128e0_f64 * t8940 * t49493 * t352 + 0.23948483403727617128e0_f64 * t46686 - 0.35922725105591425692e0_f64 * t4669 * t44187 * t558;
    (t49480, t49493, t49501)
}

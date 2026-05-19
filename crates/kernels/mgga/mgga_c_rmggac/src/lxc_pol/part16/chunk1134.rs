//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 1134/1158 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk1134<F: Float>(t1763: F, t698: F, t2447: F, t570: F, t27055: F, t321: F, t333: F, t352: F, t44187: F, t44239: F, t46662: F, t46664: F, t46669: F, t46671: F, t46673: F, t46675: F, t46677: F, t46686: F, t4669: F, t49411: F, t5148: F, t558: F, t8940: F) -> (F, F, F) {
    let t49480 = t698 * t1763;
    let t49493 = t2447 * t570;
    let t49501 = -F::cast_from(0.35922725105591425692e0_f64) * t46662 - F::cast_from(0.23948483403727617128e0_f64) * t5148 * t44239 * t570 - F::cast_from(0.35922725105591425692e0_f64) * t27055 * t49480 * t333 - F::cast_from(0.35922725105591425692e0_f64) * t46664 - F::cast_from(0.11974241701863808564e0_f64) * t46669 - F::cast_from(0.35922725105591425692e0_f64) * t46671 - F::cast_from(0.35922725105591425692e0_f64) * t46673 - F::cast_from(0.11974241701863808564e0_f64) * t5148 * t49411 * t321 + F::cast_from(0.17961362552795712846e0_f64) * t46675 + F::cast_from(0.71845450211182851384e0_f64) * t46677 + F::cast_from(0.23948483403727617128e0_f64) * t8940 * t49493 * t352 + F::cast_from(0.23948483403727617128e0_f64) * t46686 - F::cast_from(0.35922725105591425692e0_f64) * t4669 * t44187 * t558;
    (t49480, t49493, t49501)
}

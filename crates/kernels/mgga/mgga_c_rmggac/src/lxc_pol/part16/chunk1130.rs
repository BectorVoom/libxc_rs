//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 1130/1158 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk1130<F: Float>(t2447: F, t558: F, t321: F, t326: F, t46509: F, t46512: F, t46516: F, t46523: F, t46527: F, t46531: F, t46535: F, t46539: F, t46543: F, t46554: F, t46556: F, t4669: F, t48217: F) -> (F, F) {
    let t49394 = t2447 * t558;
    let t49398 = -F::cast_from(0.2727466165424534173e-1_f64) * t46509 + F::cast_from(0.16364796992547205038e0_f64) * t46512 + F::cast_from(0.40911992481368012596e-1_f64) * t46516 - F::cast_from(0.2727466165424534173e0_f64) * t46523 - F::cast_from(0.5454932330849068346e-1_f64) * t46527 - F::cast_from(0.5454932330849068346e-1_f64) * t46531 - F::cast_from(0.40911992481368012595e-1_f64) * t46535 + F::cast_from(0.5454932330849068346e-1_f64) * t46539 + F::cast_from(0.40911992481368012595e-1_f64) * t46543 - F::cast_from(0.17961362552795712846e0_f64) * t46554 - F::cast_from(0.5987120850931904282e-1_f64) * t46556 - F::cast_from(0.59871208509319042821e-1_f64) * t326 * t48217 - F::cast_from(0.35922725105591425692e0_f64) * t4669 * t49394 * t321;
    (t49394, t49398)
}

//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 1130/1158 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk1130(t2447: f64, t558: f64, t321: f64, t326: f64, t46509: f64, t46512: f64, t46516: f64, t46523: f64, t46527: f64, t46531: f64, t46535: f64, t46539: f64, t46543: f64, t46554: f64, t46556: f64, t4669: f64, t48217: f64) -> (f64, f64) {
    let t49394 = t2447 * t558;
    let t49398 = -0.2727466165424534173e-1_f64 * t46509 + 0.16364796992547205038e0_f64 * t46512 + 0.40911992481368012596e-1_f64 * t46516 - 0.2727466165424534173e0_f64 * t46523 - 0.5454932330849068346e-1_f64 * t46527 - 0.5454932330849068346e-1_f64 * t46531 - 0.40911992481368012595e-1_f64 * t46535 + 0.5454932330849068346e-1_f64 * t46539 + 0.40911992481368012595e-1_f64 * t46543 - 0.17961362552795712846e0_f64 * t46554 - 0.5987120850931904282e-1_f64 * t46556 - 0.59871208509319042821e-1_f64 * t326 * t48217 - 0.35922725105591425692e0_f64 * t4669 * t49394 * t321;
    (t49394, t49398)
}

//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 1129/1158 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk1129(t326: f64, t46471: f64, t46473: f64, t46476: f64, t46480: f64, t46483: f64, t46486: f64, t46488: f64, t46492: f64, t46503: f64, t46505: f64, t46507: f64, t48482: f64) -> f64 {
    let t49380 = 0.32729593985094410076e0_f64 * t46471 - 0.8182398496273602519e0_f64 * t46473 - 0.16364796992547205038e0_f64 * t46476 + 0.81823984962736025192e-1_f64 * t46480 - 0.16364796992547205038e0_f64 * t46483 - 0.81823984962736025192e-1_f64 * t46486 - 0.72732431077987577947e-1_f64 * t46488 - 0.18183107769496894487e-1_f64 * t46492 - 0.11974241701863808564e0_f64 * t326 * t48482 + 0.40911992481368012596e-1_f64 * t46503 - 0.81823984962736025192e-1_f64 * t46505 + 0.16364796992547205038e0_f64 * t46507;
    t49380
}

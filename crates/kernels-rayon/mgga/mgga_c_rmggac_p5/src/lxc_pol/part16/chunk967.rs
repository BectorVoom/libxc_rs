//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 967/1158 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk967(t2100: f64, t46117: f64, t2103: f64, t46122: f64, t6441: f64, t649: f64, t7599: f64, t6421: f64, t8746: f64, t41130: f64, t6425: f64, t36103: f64, t46083: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t46133 = t2100 * t46117;
    let t46135 = t2103 * t46122;
    let t46139 = t649 * t6441;
    let t46140 = t7599 * t46139;
    let t46142 = t649 * t6421;
    let t46143 = t8746 * t46142;
    let t46146 = t41130 * t649 * t6425;
    let t46150 = t36103 * t46083;
    (t46133, t46135, t46139, t46140, t46142, t46143, t46146, t46150)
}

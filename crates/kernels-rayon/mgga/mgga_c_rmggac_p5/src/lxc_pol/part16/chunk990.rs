//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 990/1158 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk990(t46185: f64, t7829: f64, t2068: f64, t46117: f64, t2073: f64, t46122: f64, t2079: f64, t262: f64, t36: f64, t6463: f64, t27041: f64, t45568: f64) -> (f64, f64, f64, f64, f64) {
    let t46648 = t7829 * t46185;
    let t46650 = t2068 * t46117;
    let t46652 = t2073 * t46122;
    let t46656 = t2079 * t262 * t36 * t6463;
    let t46658 = t27041 * t45568;
    (t46648, t46650, t46652, t46656, t46658)
}

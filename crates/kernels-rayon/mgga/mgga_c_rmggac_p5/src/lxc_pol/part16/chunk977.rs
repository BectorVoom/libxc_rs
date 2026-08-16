//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 977/1158 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk977(t265: f64, t9908: f64, t46128: f64, t851: f64, t46176: f64, t854: f64, t3810: f64, t46184: f64, t3839: f64, t46180: f64, t2068: f64, t46129: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t46302 = t9908 * t265;
    let t46305 = t851 * t46128;
    let t46307 = t854 * t46176;
    let t46309 = t3810 * t46184;
    let t46311 = t3839 * t46180;
    let t46320 = t2068 * t46129;
    (t46302, t46305, t46307, t46309, t46311, t46320)
}

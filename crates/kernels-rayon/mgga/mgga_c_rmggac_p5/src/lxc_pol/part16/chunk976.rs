//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 976/1158 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk976(t41176: f64, t6387: f64, t3814: f64, t46184: f64, t46121: f64, t854: f64, t6444: f64, t9872: f64, t46116: f64, t793: f64, t46176: f64, t797: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t46285 = t41176 * t6387;
    let t46287 = t3814 * t46184;
    let t46289 = t854 * t46121;
    let t46291 = t6444 * t9872;
    let t46293 = t793 * t46116;
    let t46300 = t797 * t46176;
    (t46285, t46287, t46289, t46291, t46293, t46300)
}

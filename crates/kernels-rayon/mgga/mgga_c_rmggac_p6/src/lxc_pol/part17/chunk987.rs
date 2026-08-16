//! MGGA_C_RMGGAC lxc pol — lxc_pol part 17 (v4rho3sigma_8) CSE chunk 987/1111 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part17_v4rho3sigma_8_chunk987(t3839: f64, t46278: f64, t41165: f64, t6387: f64, t41262: f64, t6382: f64, t41176: f64, t3814: f64, t46184: f64, t46121: f64, t854: f64, t6444: f64, t9872: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t46279 = t3839 * t46278;
    let t46281 = t41165 * t6387;
    let t46283 = t41262 * t6382;
    let t46285 = t41176 * t6387;
    let t46287 = t3814 * t46184;
    let t46289 = t854 * t46121;
    let t46291 = t6444 * t9872;
    (t46279, t46281, t46283, t46285, t46287, t46289, t46291)
}

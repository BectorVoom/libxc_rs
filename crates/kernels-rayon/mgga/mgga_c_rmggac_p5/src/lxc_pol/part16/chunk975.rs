//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 975/1158 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk975(t3826: f64, t45418: f64, t15093: f64, t8704: f64, t2074: f64, t46068: f64, t321: f64, t9872: f64, t3839: f64, t41165: f64, t6387: f64, t41262: f64, t6382: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t46272 = t3826 * t45418;
    let t46274 = t15093 * t8704;
    let t46276 = t46068 * t2074;
    let t46278 = t9872 * t321;
    let t46279 = t3839 * t46278;
    let t46281 = t41165 * t6387;
    let t46283 = t41262 * t6382;
    (t46272, t46274, t46276, t46278, t46279, t46281, t46283)
}

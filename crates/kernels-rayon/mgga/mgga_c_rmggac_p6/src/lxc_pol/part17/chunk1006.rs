//! MGGA_C_RMGGAC lxc pol — lxc_pol part 17 (v4rho3sigma_8) CSE chunk 1006/1111 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part17_v4rho3sigma_8_chunk1006(t262: f64, t46537: f64, t36274: f64, t10166: f64, t352: f64, t35929: f64, t5840: f64, t665: f64, t1737: f64, t664: f64, t46261: f64, t5271: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t46538 = t262 * t46537;
    let t46539 = t36274 * t46538;
    let t46541 = t10166 * t352;
    let t46542 = t262 * t46541;
    let t46543 = t35929 * t46542;
    let t46547 = t665 * t5840;
    let t46550 = t664 * t1737;
    let t46554 = t5271 * t46261;
    (t46538, t46539, t46541, t46542, t46543, t46547, t46550, t46554)
}

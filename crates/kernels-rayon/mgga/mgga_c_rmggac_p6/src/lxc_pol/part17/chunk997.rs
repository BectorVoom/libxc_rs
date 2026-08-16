//! MGGA_C_RMGGAC lxc pol — lxc_pol part 17 (v4rho3sigma_8) CSE chunk 997/1111 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part17_v4rho3sigma_8_chunk997(t262: f64, t46258: f64, t7829: f64, t352: f64, t9876: f64, t7782: f64, t10122: f64, t321: f64, t7788: f64, t333: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t46412 = t262 * t46258;
    let t46413 = t7829 * t46412;
    let t46415 = t9876 * t352;
    let t46416 = t262 * t46415;
    let t46417 = t7782 * t46416;
    let t46419 = t10122 * t321;
    let t46420 = t262 * t46419;
    let t46421 = t7788 * t46420;
    let t46423 = t10122 * t333;
    let t46424 = t262 * t46423;
    let t46425 = t7782 * t46424;
    let t46427 = t10122 * t352;
    (t46412, t46413, t46415, t46416, t46417, t46419, t46420, t46421, t46423, t46424, t46425, t46427)
}

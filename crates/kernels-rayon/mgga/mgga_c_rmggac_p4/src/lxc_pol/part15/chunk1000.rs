//! MGGA_C_RMGGAC lxc pol — lxc_pol part 15 (v4rho3sigma_6) CSE chunk 1000/1110 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part15_v4rho3sigma_6_chunk1000(t570: f64, t8712: f64, t262: f64, t7782: f64, t44733: f64, t7785: f64, t44737: f64, t7788: f64, t5144: f64, t8946: f64, t5267: f64, t5888: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t46453 = t8712 * t570;
    let t46454 = t262 * t46453;
    let t46455 = t7782 * t46454;
    let t46457 = t7785 * t44733;
    let t46459 = t7788 * t44737;
    let t46462 = t8946 * t5144;
    let t46465 = t8946 * t5267;
    let t46468 = t8946 * t5888;
    (t46453, t46454, t46455, t46457, t46459, t46462, t46465, t46468)
}

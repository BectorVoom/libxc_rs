//! MGGA_C_RMGGAC lxc pol — lxc_pol part 15 (v4rho3sigma_6) CSE chunk 877/963 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part15_v4rho3sigma_6_chunk877<F: Float>(t570: F, t8712: F, t262: F, t7782: F, t44733: F, t7785: F, t44737: F, t7788: F, t5144: F, t8946: F, t5267: F, t5888: F, t41404: F, t46106: F, t40999: F, t46109: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t46453 = t8712 * t570;
    let t46454 = t262 * t46453;
    let t46455 = t7782 * t46454;
    let t46457 = t7785 * t44733;
    let t46459 = t7788 * t44737;
    let t46462 = t8946 * t5144;
    let t46465 = t8946 * t5267;
    let t46468 = t8946 * t5888;
    let t46471 = t41404 * t46106;
    let t46473 = t40999 * t46109;
    (t46453, t46454, t46455, t46457, t46459, t46462, t46465, t46468, t46471, t46473)
}

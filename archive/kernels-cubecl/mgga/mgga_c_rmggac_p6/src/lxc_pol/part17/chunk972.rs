//! MGGA_C_RMGGAC lxc pol — lxc_pol part 17 (v4rho3sigma_8) CSE chunk 972/1111 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part17_v4rho3sigma_8_chunk972<F: Float>(t6434: F, t649: F, t7599: F, t6394: F, t36119: F, t6397: F, t41130: F, t6400: F, t8746: F, t6382: F, t36107: F, t6387: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t46092 = t649 * t6434;
    let t46093 = t7599 * t46092;
    let t46095 = t649 * t6394;
    let t46096 = t36119 * t46095;
    let t46098 = t649 * t6397;
    let t46099 = t41130 * t46098;
    let t46101 = t649 * t6400;
    let t46102 = t8746 * t46101;
    let t46106 = t649 * t6382;
    let t46107 = t36107 * t46106;
    let t46109 = t649 * t6387;
    (t46092, t46093, t46095, t46096, t46098, t46099, t46101, t46102, t46106, t46107, t46109)
}

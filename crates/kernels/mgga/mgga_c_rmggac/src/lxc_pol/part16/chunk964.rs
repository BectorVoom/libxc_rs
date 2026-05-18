//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 964/1158 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk964<F: Float>(t36107: F, t46083: F, t6412: F, t649: F, t8764: F, t6449: F, t7599: F, t6434: F, t6394: F, t36119: F, t6397: F, t41130: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t46084 = t36107 * t46083;
    let t46086 = t649 * t6412;
    let t46087 = t8764 * t46086;
    let t46089 = t649 * t6449;
    let t46090 = t7599 * t46089;
    let t46092 = t649 * t6434;
    let t46093 = t7599 * t46092;
    let t46095 = t649 * t6394;
    let t46096 = t36119 * t46095;
    let t46098 = t649 * t6397;
    let t46099 = t41130 * t46098;
    (t46084, t46086, t46087, t46089, t46090, t46092, t46093, t46095, t46096, t46098, t46099)
}

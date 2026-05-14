//! MGGA_C_RMGGAC lxc pol — lxc_pol part 15 (v4rho3sigma_6) CSE chunk 857/963 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part15_v4rho3sigma_6_chunk857<F: Float>(t36: F, t6376: F, t262: F, t2118: F, t22: F, t30526: F, t9885: F, t1734: F, t265: F, t2100: F, t46117: F, t2103: F, t6441: F, t649: F, t7599: F, t6421: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t46121 = t36 * t6376;
    let t46122 = t262 * t46121;
    let t46123 = t2118 * t46122;
    let t46126 = t30526 * t22 * t9885;
    let t46128 = t265 * t1734;
    let t46129 = t262 * t46128;
    let t46130 = t2100 * t46129;
    let t46133 = t2100 * t46117;
    let t46135 = t2103 * t46122;
    let t46139 = t649 * t6441;
    let t46140 = t7599 * t46139;
    let t46142 = t649 * t6421;
    (t46121, t46122, t46123, t46126, t46128, t46129, t46130, t46133, t46135, t46139, t46140, t46142)
}

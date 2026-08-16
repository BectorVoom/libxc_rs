//! MGGA_C_RMGGAC lxc pol — lxc_pol part 15 (v4rho3sigma_6) CSE chunk 1006/1110 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part15_v4rho3sigma_6_chunk1006<F: Float>(t262: F, t46537: F, t36274: F, t10166: F, t352: F, t35929: F, t5840: F, t665: F, t1737: F, t664: F, t46261: F, t5271: F) -> (F, F, F, F, F, F, F, F) {
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

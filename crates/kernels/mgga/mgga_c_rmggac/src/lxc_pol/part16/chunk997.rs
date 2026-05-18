//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 997/1158 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk997<F: Float>(t1737: F, t352: F, t2060: F, t4044: F, t36978: F, t6382: F, t656: F, t36629: F, t6387: F, t36471: F, t6530: F, t2604: F, t9812: F) -> (F, F, F, F, F, F) {
    let t46846 = t1737 * t352;
    let t46848 = t4044 * t2060 * t46846;
    let t46853 = t36978 * t656 * t6382;
    let t46856 = t36629 * t656 * t6387;
    let t46859 = t36471 * t656 * t6530;
    let t46861 = t2604 * t9812;
    (t46846, t46848, t46853, t46856, t46859, t46861)
}

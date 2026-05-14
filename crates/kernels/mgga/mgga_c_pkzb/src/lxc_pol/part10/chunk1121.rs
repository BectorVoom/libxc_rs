//! MGGA_C_PKZB lxc pol — lxc_pol part 10 (v4rho4_2) CSE chunk 1121/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part10_v4rho4_2_chunk1121<F: Float>(t10167: F, t10188: F, t405: F, t921: F, t758: F, t3857: F, t754: F, t46: F, t915: F, t2099: F, t3882: F, t918: F, t3898: F, t6416: F, t8254: F, t2371: F, t3223: F) -> (F, F, F, F, F, F, F, F, F) {
    let t10189 = t10167 + t10188;
    let t10191 = t405 * t10189 * t921;
    let t10192 = t758 * t10191;
    let t10195 = t3857 * t754;
    let t10196 = t10195 * t46;
    let t10197 = t915 * t10196;
    let t10200 = t2099 * t3882;
    let t10201 = t918 * t10200;
    let t10204 = t6416 * t3898;
    let t10205 = t8254 * t10204;
    let t10208 = t2371 * t3223;
    (t10189, t10191, t10192, t10195, t10197, t10201, t10204, t10205, t10208)
}

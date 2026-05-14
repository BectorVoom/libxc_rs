//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 889/1345 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk889<F: Float>(t1967: F, t3068: F, t1211: F, t6116: F, t1975: F, t6127: F, t1947: F, t82: F, t79: F, t3073: F, t3086: F, t3087: F, t3093: F, t3096: F, t3099: F, t6088: F, t623: F, t627: F, t74: F, t8061: F, t8080: F, t81: F, t8102: F, t8103: F) -> (F, F, F, F, F) {
    let t8106 = t1967 * t3068;
    let t8109 = t6116 * t1211;
    let t8117 = t1975 * t3068;
    let t8122 = t6127 * t1211;
    let t8125 = t1947 * t82;
    let t8130 = t79 * t1947;
    let t8138 = 15.0 / 2.0 * t8102 * t8103 - 4.0 * t8106 * t3087 - 5.0 / 2.0 * t8109 * t8103 - 2.0 * t3086 * t6088 + t623 * t8061 * t81 / 2.0 + t8117 * t3087 / 2.0 + t3093 * t6088 / 4.0 + t8122 * t8103 / 8.0 - 4.0 * t8125 * t1211 - 8.0 * t3096 * t3068 - t8130 * t3073 - 2.0 * t3099 * t8080 - 4.0 * t627 * t8061 - t74 * t8061 * t81;
    (t8109, t8122, t8125, t8130, t8138)
}

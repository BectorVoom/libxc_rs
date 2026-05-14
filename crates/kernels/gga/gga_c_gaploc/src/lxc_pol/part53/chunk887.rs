//! GGA_C_GAPLOC lxc pol — lxc_pol part 53 (v4rhosigma3_18) CSE chunk 887/923 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part53_v4rhosigma3_18_chunk887<F: Float>(t42687: F, t42689: F, t42691: F, t42694: F, t42698: F, t42703: F, t42706: F, t42708: F, t42712: F, t42715: F, t42718: F, t42719: F, t42722: F, t46956: F, t46961: F, t46963: F, t46967: F, t46970: F) -> (F,) {
    let t50958 = -t46956 + t42687 - t42689 - t42691 - t42694 + t42698 + t42703 + t42706 + t42708 + t42712 + t42715 + t42718 + t42719 + t42722 + 0.15176003539027279787e0 * t46961 - 0.17073003981405689759e0 * t46963 - 0.17073003981405689759e0 * t46967 - 0.17073003981405689759e0 * t46970;
    (t50958,)
}

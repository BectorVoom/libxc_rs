//! MGGA_C_R2SCAN lxc pol — lxc_pol part 6 (v4rho4_1) CSE chunk 1316/1462 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part6_v4rho4_1_chunk1316<F: Float>(t24858: F, t2666: F, t5135: F, t2201: F, t2202: F, t8279: F, t1551: F, t20254: F, t20264: F, t20270: F, t20279: F, t23007: F, t24833: F, t24836: F, t24839: F, t24840: F, t24847: F, t2567: F, t2572: F, t2582: F, t360: F, t5139: F, t6250: F, t6450: F, t7433: F, t7987: F) -> (F,) {
    let t24859 = 0.38140175656238781678e1 * t24858;
    let t24860 = t5135 * t2666;
    let t24864 = t2201 * t8279 * t2202;
    let t24869 = 0.12713391885412927226e1 * t20254 - 0.41917145582815912122e0 * t24833 - 0.34672886960217074253e0 * t24836 + t24839 + 0.7801399566048841707e1 * t24840 * t360 * t2567 * t6450 + 0.39006997830244208535e0 * t7987 * t6250 + 0.2600466522016280569e1 * t23007 * t360 * t2572 * t24847 - 0.13002332610081402845e0 * t2582 * t360 * t7433 * t1551 - t24859 - 0.2600466522016280569e0 * t24860 * t5139 - 0.34930954652346593433e-1 * t24864 - 0.69345773920434148506e0 * t20264 + 0.11557628986739024751e0 * t20270 + 0.64025200389650807209e-1 * t20279;
    (t24869,)
}

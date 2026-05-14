//! MGGA_C_R2SCAN lxc pol — lxc_pol part 6 (v4rho4_1) CSE chunk 1304/1462 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part6_v4rho4_1_chunk1304<F: Float>(t20646: F, t5073: F, t113: F, t19999: F, t20021: F, t20024: F, t2139: F, t22721: F, t24575: F, t24583: F, t24589: F, t24600: F, t24609: F, t24615: F, t2567: F, t2572: F, t360: F, t495: F, t5066: F, t5074: F, t5137: F, t6152: F, t7433: F, t7944: F, t8007: F) -> (F, F) {
    let t24616 = t20646 * t5073;
    let t24621 = t24575 + 0.13002332610081402845e0 * t2139 * t360 * t2567 * t5074 + t24583 + 0.2600466522016280569e0 * t22721 * t360 * t2572 * t113 * t5137 + 0.39006997830244208535e0 * t2139 * t360 * t24589 * t495 + 0.39006997830244208535e0 * t6152 * t8007 + 0.13002332610081402845e0 * t2139 * t360 * t2567 * t5066 + 0.38415120233790484326e0 * t24600 - 0.38415120233790484326e0 * t19999 + 0.39006997830244208535e0 * t2139 * t360 * t7433 * t7944 - 0.34672886960217074253e0 * t20021 - 0.10401866088065122276e1 * t20024 + 0.13002332610081402845e0 * t2139 * t360 * t2572 * t24609 + 0.10401866088065122276e1 * t24615 * t360 * t2572 * t24616;
    (t24616, t24621)
}

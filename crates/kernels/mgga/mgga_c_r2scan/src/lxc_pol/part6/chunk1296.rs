//! MGGA_C_R2SCAN lxc pol — lxc_pol part 6 (v4rho4_1) CSE chunk 1296/1462 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part6_v4rho4_1_chunk1296<F: Float>(t24384: F, t24407: F, t1551: F, t1570: F, t19807: F, t19845: F, t2122: F, t2124: F, t2133: F, t2169: F, t24136: F, t24165: F, t24208: F, t24214: F, t24245: F, t24318: F, t2557: F, t2562: F, t2598: F, t360: F, t5074: F, t5108: F, t5109: F, t551: F, t552: F, t566: F, t6106: F, t6121: F, t6134: F, t6139: F, t6435: F, t7240: F, t7337: F, t7378: F, t7433: F, t7435: F, t7921: F, t8012: F, t8022: F, t921: F) -> (F, F) {
    let t24409 = t24384 / 2.0 + t24407 / 2.0;
    let t24428 = 0.78013995660488417067e0 * t2598 * t5109 * t24245 - 0.7801399566048841707e0 * t6139 * t5109 * t24318 + 0.13002332610081402845e0 * t2133 * t5109 * t7921 * t1551 - 0.39006997830244208535e0 * t5108 * t5109 * t921 * t6435 - 0.32927245914677557992e0 * t2122 * t7337 * t24136 + 0.98781737744032673978e0 * t2122 * t24208 * t24214 + 0.52009330440325611378e0 * t19845 * t5109 * t24165 * t6134 + 0.7801399566048841707e0 * t19807 * t5109 * t24165 * t6121 - 0.39006997830244208535e0 * t2169 * t7240 - 0.13002332610081402845e0 * t566 * t551 * t552 * t24409 + 0.49390868872016336988e0 * t2557 * t2124 * t8012 * t1570 + 0.52009330440325611378e0 * t8022 * t7435 + 0.43341108700271342816e-1 * t2133 * t360 * t2562 * t5074 - 0.15602799132097683414e1 * t6106 * t360 * t7433 * t7378;
    (t24409, t24428)
}

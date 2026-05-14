//! MGGA_C_R2SCAN lxc pol — lxc_pol part 8 (v4rho4_3) CSE chunk 1341/1467 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part8_v4rho4_3_chunk1341<F: Float>(t113: F, t32490: F, t6086: F, t6093: F, t537: F, t9880: F, t277: F, t9947: F, t9937: F, t10060: F, t2122: F, t2124: F, t2133: F, t2139: F, t22780: F, t23007: F, t2531: F, t2551: F, t2567: F, t2573: F, t2582: F, t29070: F, t29074: F, t32319: F, t32664: F, t360: F, t495: F, t7461: F, t7984: F, t8780: F, t8792: F, t8820: F, t8827: F, t9212: F) -> (F, F, F) {
    let t32871 = t32490 * t113;
    let t32873 = t6093 * t6086 * t32871;
    let t32885 = t537 * t9880;
    let t32896 = t277 * t9947;
    let t32911 = t277 * t9937;
    let t32918 = 0.52396431978519890152e-1 * t32873 - 0.26004665220162805689e0 * t22780 * t10060 + 0.54878743191129263322e-1 * t2122 * t2124 * t32664 * t2551 - 0.13002332610081402845e0 * t2582 * t360 * t8820 * t2531 + 0.54878743191129263322e-1 * t2122 * t2124 * t32885 * t495 - 0.31205598264195366828e1 * t7461 * t360 * t2567 * t9212 - 0.13002332610081402845e0 * t8792 * t8780 + 0.43341108700271342816e-1 * t2133 * t360 * t32896 * t2573 + 0.13002332610081402845e0 * t2139 * t360 * t32896 * t2551 - 0.43341108700271342816e-1 * t2582 * t360 * t32319 * t495 + 0.13002332610081402845e0 * t7984 * t8827 + 0.2600466522016280569e1 * t23007 * t360 * t32911 * t495 - 0.76280351312477563357e1 * t29070 - 0.38140175656238781679e1 * t29074;
    (t32871, t32896, t32918)
}

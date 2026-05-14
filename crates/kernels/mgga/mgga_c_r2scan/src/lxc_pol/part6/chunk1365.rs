//! MGGA_C_R2SCAN lxc pol — lxc_pol part 6 (v4rho4_1) CSE chunk 1365/1462 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part6_v4rho4_1_chunk1365<F: Float>(t122: F, t512: F, t10855: F, t110: F, t25566: F, t20096: F, t7418: F, t20818: F, t24999: F, t2294: F, t6583: F, t8111: F, t25618: F, t6448: F, t1554: F, t19990: F, t2122: F, t2124: F, t2139: F, t2254: F, t24877: F, t2545: F, t2550: F, t2551: F, t2557: F, t2569: F, t2582: F, t360: F, t5066: F, t6293: F, t7245: F, t7378: F, t7433: F, t7463: F, t7503: F, t7518: F, t7528: F, t8029: F, t8289: F) -> (F, F, F) {
    let t25850 = t512 * t122;
    let t25851 = t10855 * t110;
    let t25852 = t25850 * t25851;
    let t25853 = t25852 * t25566;
    let t25855 = t20096 * t7418;
    let t25871 = t20818 * t24999;
    let t25872 = 0.86743646395112941037e-3 * t25871;
    let t25880 = t6583 * t2294 * t8111;
    let t25882 = t6448 * t25618;
    let t25893 = -0.13002332610081402845e0 * t8289 * t7528 + 0.58544643236296698111e-1 * t25853 + 0.31806003678208078381e-2 * t25855 - 0.13002332610081402845e0 * t7245 * t2254 - 0.49390868872016336989e0 * t6293 * t2124 * t7503 * t7378 - 0.13002332610081402845e0 * t2582 * t360 * t7433 * t1554 + 0.54878743191129263322e-1 * t2122 * t2124 * t2545 * t5066 + t25872 + 0.39006997830244208535e0 * t19990 * t2569 + 0.39006997830244208535e0 * t2139 * t360 * t24877 * t2551 + 0.69345773920434148506e0 * t25880 - 0.31205598264195366828e1 * t25882 * t7463 - 0.27439371595564631661e-1 * t2557 * t2124 * t2550 * t5066 - 0.78013995660488417068e0 * t8029 * t360 * t7433 * t7518;
    (t25850, t25852, t25893)
}

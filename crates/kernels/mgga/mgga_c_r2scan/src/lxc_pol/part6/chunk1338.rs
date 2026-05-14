//! MGGA_C_R2SCAN lxc pol — lxc_pol part 6 (v4rho4_1) CSE chunk 1338/1462 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part6_v4rho4_1_chunk1338<F: Float>(t1591: F, t7983: F, t1551: F, t19820: F, t19987: F, t20040: F, t20366: F, t20371: F, t20376: F, t20511: F, t21003: F, t24192: F, t24204: F, t2533: F, t2564: F, t2567: F, t2582: F, t2612: F, t360: F, t5108: F, t5109: F, t5111: F, t6106: F, t6198: F, t6364: F, t6583: F, t7327: F, t7330: F, t7353: F, t8110: F, t921: F) -> (F,) {
    let t25243 = t1591 * t7983;
    let t25260 = -0.17465477326173296717e-1 * t20366 - 0.58218257753910989057e-2 * t20371 + 0.7801399566048841707e0 * t21003 * t360 * t2567 * t6364 + 0.13002332610081402845e0 * t19987 * t2564 + 0.34930954652346593433e-1 * t20376 - 0.26004665220162805689e0 * t6583 * t5109 * t921 * t6198 - 0.39006997830244208535e0 * t5108 * t5109 * t2612 * t1551 - 0.52009330440325611378e0 * t20040 * t7327 - 0.7801399566048841707e0 * t25243 * t5111 - 0.13002332610081402845e0 * t2582 * t5109 * t24204 - 0.26004665220162805689e0 * t6583 * t5109 * t2533 * t8110 - 0.31205598264195366828e1 * t20511 * t7330 - 0.15602799132097683414e1 * t6106 * t5109 * t24192 - 0.7801399566048841707e0 * t19820 * t7353;
    (t25260,)
}

//! MGGA_C_R2SCAN lxc pol — lxc_pol part 8 (v4rho4_3) CSE chunk 1329/1467 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part8_v4rho4_3_chunk1329<F: Float>(t3055: F, t910: F, t10090: F, t495: F, t2551: F, t9950: F, t2122: F, t2133: F, t2139: F, t2573: F, t2591: F, t27741: F, t27744: F, t27746: F, t32340: F, t32344: F, t32365: F, t32474: F, t32485: F, t5108: F, t5109: F, t6106: F, t6132: F, t6139: F, t6293: F, t7321: F, t7984: F, t8029: F, t8737: F) -> (F, F, F, F) {
    let t32490 = t910 * t3055;
    let t32497 = t10090 * t495;
    let t32504 = t9950 * t2551;
    let t32514 = 0.16463622957338778996e0 * t2122 * t7321 * t32474 - 0.7801399566048841707e0 * t6139 * t5109 * t32365 + 0.13002332610081402845e0 * t2133 * t5109 * t9950 * t2573 - 0.2600466522016280569e0 * t6132 * t5109 * t32485 * t2591 - 0.39006997830244208535e0 * t5108 * t5109 * t32490 * t2573 + 0.13869154784086829701e1 * t27741 + 0.41607464352260489104e1 * t27744 - 0.49390868872016336991e0 * t6293 * t7321 * t32497 - 0.15602799132097683414e1 * t6106 * t5109 * t32340 + 0.39006997830244208535e0 * t2139 * t5109 * t32504 - 0.78013995660488417067e0 * t8029 * t5109 * t32344 + 0.26004665220162805689e0 * t7984 * t8737 + 0.19207560116895242163e0 * t27746;
    (t32490, t32497, t32504, t32514)
}

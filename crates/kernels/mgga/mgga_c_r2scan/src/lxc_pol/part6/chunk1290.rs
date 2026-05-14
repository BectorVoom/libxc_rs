//! MGGA_C_R2SCAN lxc pol — lxc_pol part 6 (v4rho4_1) CSE chunk 1290/1462 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part6_v4rho4_1_chunk1290<F: Float>(t2551: F, t7202: F, t1551: F, t2625: F, t21003: F, t2133: F, t2139: F, t22780: F, t24188: F, t24192: F, t24196: F, t24204: F, t24208: F, t24210: F, t24214: F, t24218: F, t2557: F, t2573: F, t2654: F, t5108: F, t5109: F, t6106: F, t6127: F, t6132: F, t6139: F, t6293: F, t7321: F, t7346: F, t7921: F) -> (F, F, F) {
    let t24224 = t7202 * t2551;
    let t24228 = t2625 * t1551;
    let t24232 = -0.39006997830244208535e0 * t5108 * t5109 * t2654 * t1551 - 0.52009330440325611378e0 * t22780 * t7346 - 0.15602799132097683414e1 * t6139 * t5109 * t24188 - 0.49390868872016336991e0 * t6293 * t7321 * t24192 - 0.7801399566048841707e0 * t6139 * t5109 * t24196 - 0.26004665220162805689e0 * t6132 * t5109 * t7921 * t6127 - 0.82318114786693894983e-1 * t2557 * t7321 * t24204 - 0.49390868872016336991e0 * t2557 * t24208 * t24210 + 0.23404198698146525121e1 * t21003 * t5109 * t24214 - 0.76830240467580968651e0 * t24218 + 0.13002332610081402845e0 * t2133 * t5109 * t7202 * t2573 + 0.39006997830244208535e0 * t2139 * t5109 * t24224 - 0.15602799132097683414e1 * t6106 * t5109 * t24228;
    (t24224, t24228, t24232)
}

//! MGGA_C_R2SCAN lxc pol — lxc_pol part 8 (v4rho4_3) CSE chunk 1333/1467 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part8_v4rho4_3_chunk1333<F: Float>(t495: F, t9950: F, t10017: F, t10051: F, t10125: F, t20127: F, t2122: F, t2133: F, t2139: F, t2531: F, t2551: F, t2557: F, t26029: F, t27899: F, t27910: F, t32348: F, t32357: F, t32497: F, t32504: F, t32590: F, t360: F, t5108: F, t5109: F, t6106: F, t6139: F, t6149: F, t6152: F, t7321: F, t7987: F, t8735: F, t8746: F, t8820: F, t8834: F) -> (F, F) {
    let t32626 = t9950 * t495;
    let t32636 = 0.39006997830244208535e0 * t6152 * t10017 - 0.7801399566048841707e0 * t6139 * t360 * t8820 * t32590 - 0.20803732176130244552e1 * t27899 + 0.39006997830244208535e0 * t7987 * t8834 - 0.20803732176130244552e1 * t27910 + 0.13002332610081402845e0 * t6149 * t10051 + 0.2600466522016280569e0 * t2133 * t5109 * t8735 * t2531 - 0.15602799132097683414e1 * t6106 * t5109 * t32497 + 0.16463622957338778997e0 * t2122 * t7321 * t32504 - 0.39006997830244208535e0 * t5108 * t5109 * t32348 * t2551 + 0.7801399566048841707e0 * t2139 * t5109 * t32357 - 0.82318114786693894983e-1 * t2557 * t7321 * t32626 - 0.52009330440325611378e0 * t26029 * t8746 + 0.7801399566048841707e0 * t20127 * t5109 * t10125 * t495;
    (t32626, t32636)
}

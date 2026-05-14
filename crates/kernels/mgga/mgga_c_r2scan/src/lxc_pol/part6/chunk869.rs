//! MGGA_C_R2SCAN lxc pol — lxc_pol part 6 (v4rho4_1) CSE chunk 869/1462 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part6_v4rho4_1_chunk869<F: Float>(t1584: F, t1588: F, t2122: F, t2136: F, t2139: F, t2142: F, t2557: F, t2598: F, t576: F, t6115: F, t6119: F, t6123: F, t6129: F, t6132: F, t6136: F, t6139: F, t6141: F, t6145: F, t6149: F, t6152: F, t6157: F, t6164: F, t6168: F, t6171: F, t6175: F, t6178: F, t6182: F) -> (F,) {
    let t6187 = -0.20803732176130244552e1 * t6115 - 0.76830240467580968651e0 * t6119 - 0.32927245914677557992e0 * t2122 * t6123 + 0.16463622957338778996e0 * t2557 * t6129 - 0.26004665220162805689e0 * t6132 * t6136 - 0.7801399566048841707e0 * t6139 * t6141 + 0.26004665220162805689e0 * t2598 * t6145 + 0.26004665220162805689e0 * t6149 * t2136 + 0.7801399566048841707e0 * t6152 * t2142 - 0.32927245914677557992e-1 * t6157 - t6164 + 0.1047928639570397803e0 * t6168 + 0.39006997830244208535e0 * t2139 * t6171 + 0.16463622957338778996e0 * t2122 * t6175 + 0.19207560116895242163e0 * t6178 - 0.13002332610081402845e0 * t6182 * t576 - 0.13002332610081402845e0 * t1584 * t1588;
    (t6187,)
}

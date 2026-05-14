//! MGGA_C_R2SCAN lxc pol — lxc_pol part 8 (v4rho4_3) CSE chunk 1414/1467 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part8_v4rho4_3_chunk1414<F: Float>(t277: F, t9981: F, t2530: F, t6363: F, t10024: F, t10065: F, t1569: F, t21029: F, t2122: F, t2124: F, t22721: F, t22749: F, t22783: F, t2557: F, t2590: F, t2719: F, t2841: F, t28418: F, t30371: F, t30374: F, t30377: F, t32664: F, t34020: F, t360: F, t495: F, t6132: F, t6152: F, t8012: F, t8820: F, t8847: F, t9115: F, t9533: F, t9991: F) -> (F, F) {
    let t34316 = t277 * t9981;
    let t34321 = t6363 * t2530;
    let t34334 = -0.26004665220162805689e0 * t6132 * t360 * t8820 * t1569 * t2719 + 0.39006997830244208535e0 * t6152 * t9991 - 0.19756347548806534796e1 * t22749 * t2124 * t2841 * t9115 - 0.7801399566048841707e0 * t22783 * t10065 + 0.16463622957338778996e0 * t2557 * t2124 * t8012 * t10024 - 0.27439371595564631661e-1 * t2557 * t2124 * t32664 * t495 + 0.16463622957338778996e0 * t2557 * t2124 * t2590 * t34020 + 0.2600466522016280569e0 * t22721 * t360 * t34316 * t495 - 0.49390868872016336991e0 * t2557 * t2124 * t9533 * t34321 + 0.16463622957338778996e0 * t2122 * t2124 * t8847 * t28418 - 0.89443204944342177673e-3 * t21029 + 0.69861909304693186866e-1 * t30371 - 0.32927245914677557992e-1 * t30374 - 0.32927245914677557992e-1 * t30377;
    (t34321, t34334)
}

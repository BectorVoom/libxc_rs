//! MGGA_C_R2SCAN lxc pol — lxc_pol part 6 (v4rho4_1) CSE chunk 1334/1462 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part6_v4rho4_1_chunk1334<F: Float>(t2294: F, t7512: F, t7513: F, t146: F, t5094: F, t774: F, t7616: F, t560: F, t7977: F, t2148: F, t6165: F, t113: F, t1570: F, t20340: F, t2124: F, t2133: F, t22749: F, t22783: F, t24847: F, t2530: F, t2550: F, t2557: F, t2567: F, t2572: F, t360: F, t6139: F, t6145: F, t6359: F, t6435: F, t6566: F, t6580: F, t7369: F, t7428: F, t7433: F, t7461: F, t7518: F, t8792: F, t9521: F) -> (F,) {
    let t25141 = t7512 * t2294 * t7513;
    let t25169 = t146 * t5094 * t774;
    let t25170 = t25169 * t7616;
    let t25172 = t7977 * t560;
    let t25174 = t6165 * t2148 * t25172;
    let t25176 = -0.7801399566048841707e0 * t22783 * t7369 - 0.13002332610081402845e0 * t8792 * t6580 - 0.7801399566048841707e0 * t6139 * t360 * t7977 * t1570 + 0.41607464352260489103e1 * t25141 + 0.26004665220162805689e0 * t9521 * t6145 + 0.13002332610081402845e0 * t2133 * t360 * t7433 * t7428 + 0.43341108700271342816e-1 * t2133 * t360 * t2572 * t113 * t6566 - 0.49390868872016336989e0 * t2557 * t2124 * t6359 * t2530 * t7518 + 0.65854491829355115988e0 * t22749 * t2124 * t2550 * t24847 - 0.15602799132097683414e1 * t7461 * t360 * t2567 * t6435 + 0.38087975358139160777e-1 * t20340 + 0.41917145582815912122e0 * t25170 + 0.1047928639570397803e0 * t25174;
    (t25176,)
}

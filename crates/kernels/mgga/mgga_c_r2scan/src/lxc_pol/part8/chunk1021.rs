//! MGGA_C_R2SCAN lxc pol — lxc_pol part 8 (v4rho4_3) CSE chunk 1021/1467 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part8_v4rho4_3_chunk1021<F: Float>(t10010: F, t2124: F, t8847: F, t10007: F, t2122: F, t2139: F, t2582: F, t2667: F, t3101: F, t3116: F, t3218: F, t5136: F, t6106: F, t6293: F, t6522: F, t6528: F, t7235: F, t7237: F, t7259: F, t7263: F, t7298: F, t7984: F, t7987: F, t9978: F, t9983: F, t9987: F, t9991: F, t9995: F, t9999: F) -> (F, F) {
    let t10012 = t2124 * t8847 * t10010;
    let t10015 = -0.38140175656238781678e1 * t7235 - 0.12805040077930161442e1 * t7237 - 0.19043987679069580388e-1 * t7259 - 0.57131963037208741166e-1 * t7263 + 0.24393601348456957547e-3 * t7298 - 0.13002332610081402845e0 * t2667 * t3218 - 0.65854491829355115988e0 * t6522 * t9978 - 0.2600466522016280569e0 * t5136 * t9983 - 0.2600466522016280569e1 * t6528 * t9987 + 0.39006997830244208535e0 * t2139 * t9991 - 0.13002332610081402845e0 * t2582 * t9995 - 0.15602799132097683414e1 * t6106 * t9999 + 0.26004665220162805689e0 * t7984 * t3116 + 0.7801399566048841707e0 * t7987 * t3101 - 0.49390868872016336989e0 * t6293 * t10007 + 0.16463622957338778996e0 * t2122 * t10012;
    (t10012, t10015)
}

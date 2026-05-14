//! MGGA_C_R2SCAN lxc pol — lxc_pol part 8 (v4rho4_3) CSE chunk 1026/1467 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part8_v4rho4_3_chunk1026<F: Float>(t10090: F, t551: F, t552: F, t3216: F, t938: F, t910: F, t3016: F, t506: F, t529: F, t538: F, t9948: F, t10085: F, t1592: F, t2184: F, t2196: F, t2223: F, t2651: F, t3068: F, t3092: F, t3183: F, t527: F, t535: F, t6062: F, t6084: F, t6105: F, t7383: F, t7419: F, t8240: F, t8867: F, t8874: F, t9327: F, t948: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t10092 = t551 * t552 * t10090;
    let t10099 = t938 * t3216;
    let t10101 = t551 * t552 * t10099;
    let t10106 = t3216 * t910;
    let t10108 = t551 * t552 * t10106;
    let t10111 = t938 * t3016;
    let t10113 = t551 * t552 * t10111;
    let t10117 = t529 * t506 * t10090;
    let t10121 = t529 * t538 * t9948;
    let t10124 = -0.69345773920434148506e0 * t8867 + 0.15602799132097683414e1 * t7383 * t3183 - 0.54878743191129263322e-1 * t527 * t10085 - 0.86743646395112941037e-3 * t7419 - 0.41607464352260489103e1 * t8874 - t6062 - t6084 - t6105 + 0.15602799132097683414e1 * t2196 * t10092 - 0.13002332610081402845e0 * t9327 * t948 - 0.13002332610081402845e0 * t2651 * t3068 + 0.26004665220162805689e0 * t2184 * t10101 + 0.7801399566048841707e0 * t8240 * t3092 + 0.39006997830244208535e0 * t1592 * t10108 + 0.39006997830244208535e0 * t1592 * t10113 + 0.49390868872016336991e0 * t2223 * t10117 - 0.27439371595564631661e-1 * t535 * t10121;
    (t10092, t10099, t10101, t10106, t10108, t10111, t10113, t10117, t10121, t10124)
}

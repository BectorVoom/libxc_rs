//! MGGA_C_R2SCAN lxc pol — lxc_pol part 6 (v4rho4_1) CSE chunk 1286/1462 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part6_v4rho4_1_chunk1286<F: Float>(t24118: F, t2551: F, t1595: F, t20459: F, t2198: F, t22875: F, t24086: F, t24088: F, t24090: F, t24097: F, t24100: F, t24107: F, t495: F, t5108: F, t5109: F, t6132: F, t6221: F, t6293: F, t6340: F, t6430: F, t6437: F, t7321: F, t7338: F, t7344: F, t7383: F, t8057: F, t8240: F, t9507: F) -> (F, F) {
    let t24119 = t24118 * t2551;
    let t24131 = 0.69345773920434148506e0 * t24086 + 0.34672886960217074253e0 * t24088 + 0.15602799132097683414e1 * t24090 * t2198 - t24097 + 0.15602799132097683414e1 * t7383 * t6340 + 0.7801399566048841707e0 * t24100 * t1595 + 0.39006997830244208535e0 * t8240 * t6430 + 0.39006997830244208535e0 * t8240 * t6437 - 0.7801399566048841707e0 * t24107 * t6221 - 0.52009330440325611378e0 * t6132 * t5109 * t9507 * t7344 - 0.26004665220162805689e0 * t6132 * t5109 * t7338 * t22875 - 0.49390868872016336991e0 * t6293 * t7321 * t24119 - 0.26004665220162805689e0 * t6132 * t5109 * t7338 * t20459 - 0.7801399566048841707e0 * t5108 * t5109 * t8057 * t495;
    (t24119, t24131)
}

//! MGGA_C_R2SCAN lxc pol — lxc_pol part 8 (v4rho4_3) CSE chunk 1358/1467 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part8_v4rho4_3_chunk1358<F: Float>(t32532: F, t495: F, t113: F, t2719: F, t2651: F, t9226: F, t8240: F, t9232: F, t19845: F, t20253: F, t20499: F, t2122: F, t2139: F, t24822: F, t2591: F, t3052: F, t32445: F, t32467: F, t32474: F, t32523: F, t32675: F, t32936: F, t5109: F, t560: F, t6132: F, t6139: F, t6293: F, t6583: F, t7321: F, t7337: F, t7338: F, t8735: F) -> (F, F, F) {
    let t33196 = t32532 * t495;
    let t33209 = t113 * t2719;
    let t33221 = t2651 * t9226;
    let t33223 = t8240 * t9232;
    let t33226 = -0.26004665220162805689e0 * t6132 * t5109 * t7338 * t3052 * t560 + 0.52009330440325611378e0 * t19845 * t5109 * t32523 * t2591 - 0.78013995660488417067e0 * t6139 * t5109 * t32467 + 0.16463622957338778996e0 * t2122 * t7321 * t33196 - 0.49390868872016336988e0 * t6293 * t7321 * t32936 + 0.98781737744032673976e0 * t6293 * t7337 * t32675 + 0.39006997830244208535e0 * t2139 * t5109 * t32474 - 0.52009330440325611378e0 * t6583 * t5109 * t8735 * t33209 - 0.26004665220162805689e0 * t6583 * t5109 * t32523 * t495 + 0.31205598264195366828e1 * t20499 * t5109 * t32445 + 0.34672886960217074253e0 * t33221 - 0.20803732176130244552e1 * t33223 + 0.86743646395112941038e-4 * t24822 - t20253;
    (t33196, t33209, t33226)
}

//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 1065/1120 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk1065<F: Float>(t11345: F, t12567: F, t11523: F, t12086: F, t11199: F, t12570: F, t3262: F, t3275: F, t3472: F, t42901: F, t42855: F, t11336: F, t37327: F, t42886: F, t12929: F, t2449: F, t3787: F, t44108: F, t44110: F, t44113: F, t44115: F, t44117: F, t44120: F, t44122: F, t44125: F, t885: F) -> (F, F, F, F, F, F, F) {
    let t44127 = t12567 * t11345 / 4.0;
    let t44129 = t11523 * t12086 / 2.0;
    let t44132 = 3.0 / 4.0 * t3262 * t11199 * t12570;
    let t44135 = 5.0 / 16.0 * t3275 * t3472 * t42901;
    let t44140 = 15.0 / 16.0 * t3262 * t3472 * t42855;
    let t44143 = 15.0 / 8.0 * t37327 * t11336 * t42886;
    let t44144 = t12929 * t885 + 2.0 * t2449 * t3787 - t44108 + t44110 + t44113 - t44115 + t44117 + t44120 - t44122 - t44125 + t44127 + t44129 + t44132 - t44135 - t44140 + t44143;
    (t44127, t44129, t44132, t44135, t44140, t44143, t44144)
}

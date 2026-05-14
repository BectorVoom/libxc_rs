//! MGGA_C_REVTPSS lxc pol — lxc_pol part 34 (v4rho3sigma_9) CSE chunk 1169/1196 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part34_v4rho3sigma_9_chunk1169<F: Float>(t4173: F, t5819: F, t22738: F, t76: F, t38: F, t85037: F, t1923: F, t1926: F, t1927: F, t1928: F, t22671: F, t22688: F, t23842: F, t25132: F, t29513: F, t29525: F, t29529: F, t29532: F, t29533: F, t29551: F, t6968: F, t72: F, t7702: F, t7715: F, t7716: F, t7719: F, t7720: F, t92605: F, t92612: F) -> (F,) {
    let t114322 = t4173 * t5819;
    let t114343 = t76 * t22738;
    let t114349 = t85037 * t38;
    let t114356 = -t7702 * t29529 + t114322 * t1928 - t7702 * t29533 / 2.0 - t1923 * (-5.0 / 108.0 * t92605 * t22688 + 5.0 / 6.0 * t25132 * t23842 + 5.0 / 6.0 * t6968 * t22671 + t92612) * t72 * t1927 / 6.0 - t1923 * t29525 * t7719 / 2.0 - t1923 * t7715 * t29532 / 2.0 - t1923 * t1926 * t114343 / 6.0 + t29551 * t7716 + t29551 * t7720 - t114349 * t1928 / 6.0 - t29513 * t7716 / 2.0 - t29513 * t7720 / 2.0;
    (t114356,)
}

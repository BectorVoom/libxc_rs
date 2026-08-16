//! MGGA_C_REVTPSS lxc pol — lxc_pol part 34 (v4rho3sigma_9) CSE chunk 1312/1341 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part34_v4rho3sigma_9_chunk1312(t4173: f64, t5819: f64, t22738: f64, t76: f64, t38: f64, t85037: f64, t1923: f64, t1926: f64, t1927: f64, t1928: f64, t22671: f64, t22688: f64, t23842: f64, t25132: f64, t29513: f64, t29525: f64, t29529: f64, t29532: f64, t29533: f64, t29551: f64, t6968: f64, t72: f64, t7702: f64, t7715: f64, t7716: f64, t7719: f64, t7720: f64, t92605: f64, t92612: f64) -> f64 {
    let t114322 = t4173 * t5819;
    let t114343 = t76 * t22738;
    let t114349 = t85037 * t38;
    let t114356 = -t7702 * t29529 + t114322 * t1928 - t7702 * t29533 / 2.0_f64 - t1923 * (-5.0_f64 / 108.0_f64 * t92605 * t22688 + 5.0_f64 / 6.0_f64 * t25132 * t23842 + 5.0_f64 / 6.0_f64 * t6968 * t22671 + t92612) * t72 * t1927 / 6.0_f64 - t1923 * t29525 * t7719 / 2.0_f64 - t1923 * t7715 * t29532 / 2.0_f64 - t1923 * t1926 * t114343 / 6.0_f64 + t29551 * t7716 + t29551 * t7720 - t114349 * t1928 / 6.0_f64 - t29513 * t7716 / 2.0_f64 - t29513 * t7720 / 2.0_f64;
    t114356
}

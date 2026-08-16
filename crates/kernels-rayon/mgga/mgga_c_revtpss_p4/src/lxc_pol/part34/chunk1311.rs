//! MGGA_C_REVTPSS lxc pol — lxc_pol part 34 (v4rho3sigma_9) CSE chunk 1311/1341 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part34_v4rho3sigma_9_chunk1311(t4173: f64, t5826: f64, t1493: f64, t5872: f64, t77: f64, t22742: f64, t84: f64, t5825: f64, t22672: f64, t603: f64, t108753: f64, t108757: f64, t1928: f64, t28127: f64, t28138: f64, t29526: f64, t29548: f64, t29554: f64, t6958: f64, t7702: f64, t7706: f64, t7716: f64, t7720: f64) -> f64 {
    let t114296 = t4173 * t5826;
    let t114301 = t77 * t1493 * t5872;
    let t114305 = t77 * t84 * t22742;
    let t114311 = t77 * t84 * t5825;
    let t114313 = t603 * t22672;
    let t114320 = 5.0_f64 / 2.0_f64 * t28138 * t29548 + t114296 * t1928 + 5.0_f64 / 2.0_f64 * t28127 * t29548 + 5.0_f64 / 2.0_f64 * t6958 * t114301 + 5.0_f64 / 6.0_f64 * t6958 * t114305 - 5.0_f64 * t108753 * t7706 + t108757 * t114311 + t114313 * t1928 / 3.0_f64 + t29554 * t7716 + t29554 * t7720 - t7702 * t29526 / 2.0_f64;
    t114320
}

//! MGGA_C_REVTPSS lxc pol — lxc_pol part 36 (v4rho3sigma_11) CSE chunk 1371/1378 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part36_v4rho3sigma_11_chunk1371(t104379: f64, t111457: f64, t111532: f64, t111592: f64, t114246: f64, t114264: f64, t114270: f64, t114301: f64, t114305: f64, t114311: f64, t114313: f64, t1469: f64, t1923: f64, t1927: f64, t2123: f64, t22671: f64, t22688: f64, t22699: f64, t23842: f64, t26776: f64, t26792: f64, t29355: f64, t29554: f64, t30682: f64, t5819: f64, t5825: f64, t61: f64, t72: f64, t7566: f64, t7571: f64, t7706: f64, t7719: f64, t8144: f64, t8147: f64, t92612: f64, t96733: f64, t96804: f64) -> f64 {
    let t116798 = -t1923 * (-1232.0_f64 / 27.0_f64 * t22699 * t61 - 220.0_f64 / 9.0_f64 * t111592 * t1469 - 20.0_f64 / 9.0_f64 * t104379 * t5819 + 20.0_f64 / 3.0_f64 * t29355 * t5825 + 5.0_f64 / 108.0_f64 * t96733 * t22688 + 5.0_f64 / 6.0_f64 * t26776 * t23842 - 5.0_f64 / 6.0_f64 * t7571 * t22671 + t92612) * t72 * t1927 / 6.0_f64 - t1923 * t30682 * t7719 / 2.0_f64 + 35.0_f64 * t96804 * t114264 + 5.0_f64 / 2.0_f64 * t111532 * t7706 + t114270 * t2123 + 5.0_f64 / 2.0_f64 * t7566 * t114301 + 5.0_f64 / 6.0_f64 * t7566 * t114305 + t111457 * t114311 - 15.0_f64 * t26792 * t114246 + t114313 * t2123 / 3.0_f64 + t29554 * t8144 + t29554 * t8147;
    t116798
}

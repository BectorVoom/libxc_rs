//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1599/1850 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1599(t1487: f64, t1494: f64, t21686: f64, t21784: f64, t21794: f64, t22662: f64, t22665: f64, t22671: f64, t22719: f64, t22739: f64, t2299: f64, t2306: f64, t38: f64, t4227: f64, t4232: f64, t46001: f64, t46014: f64, t5819: f64, t5820: f64, t5825: f64, t5854: f64, t5855: f64, t5869: f64, t633: f64, t637: f64, t70: f64, t71: f64, t77: f64, t7719: f64, t85: f64, t85161: f64, t87107: f64, t87126: f64, t87145: f64, t87155: f64) -> f64 {
    let t87195 = -t5819 * t5854 * t85 / 2.0_f64 - t22665 * t1494 - t5820 * t5869 / 2.0_f64 + t38 * t87155 * t85 / 24.0_f64 + t22719 * t1494 / 6.0_f64 + t5855 * t5869 / 4.0_f64 + t1487 * t22739 / 6.0_f64 + t71 * t77 * (3640.0_f64 / 81.0_f64 * t46001 * t87145 - 560.0_f64 / 9.0_f64 * t21784 * t5825 + 28.0_f64 / 3.0_f64 * t2299 * t87107 + 112.0_f64 / 9.0_f64 * t4227 * t22671 - 4.0_f64 / 3.0_f64 * t633 * t87126 + 3640.0_f64 / 81.0_f64 * t46014 * t87145 + 560.0_f64 / 9.0_f64 * t21794 * t5825 + 28.0_f64 / 3.0_f64 * t2306 * t87107 + 112.0_f64 / 9.0_f64 * t4232 * t22671 + 4.0_f64 / 3.0_f64 * t637 * t87126) / 24.0_f64 - t87107 * t70 * t85 / 4.0_f64 - t85161 * t22662 - t21686 * t7719 * t5825;
    t87195
}

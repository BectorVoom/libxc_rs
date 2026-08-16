//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3205/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3205(t13335: f64, t13343: f64, t13346: f64, t13389: f64, t1487: f64, t1494: f64, t21690: f64, t21769: f64, t21805: f64, t2291: f64, t2292: f64, t2312: f64, t4218: f64, t4238: f64, t5819: f64, t5820: f64, t5855: f64, t5869: f64, t60717: f64, t60778: f64, t628: f64, t641: f64, t70: f64, t71: f64, t77: f64, t85: f64) -> f64 {
    let t60793 = -t60717 * t70 * t85 / 6.0_f64 + t21769 * t641 / 12.0_f64 + t5855 * t2312 / 24.0_f64 + t13335 * t1494 / 12.0_f64 + t4218 * t4238 / 6.0_f64 + t1487 * t13389 / 12.0_f64 + t2292 * t5869 / 24.0_f64 + t628 * t21805 / 12.0_f64 + t71 * t77 * t60778 / 24.0_f64 - t5819 * t2291 * t85 / 12.0_f64 - t21690 * t641 / 6.0_f64 - t5820 * t2312 / 12.0_f64 - t13343 * t1494 / 6.0_f64 - t13346 * t1494 / 3.0_f64;
    t60793
}

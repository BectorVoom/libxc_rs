//! MGGA_C_REVTPSS lxc pol — lxc_pol part 28 (v4rho3sigma_3) CSE chunk 2260/2277 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2260(t28019: f64, t531: f64, t2014: f64, t7238: f64, t25866: f64, t7898: f64, t13867: f64, t28167: f64, t8996: f64, t13872: f64, t100940: f64, t101120: f64, t101124: f64, t101407: f64, t101416: f64, t118: f64, t1310: f64, t14310: f64, t1843: f64, t2011: f64, t25169: f64, t25872: f64, t28160: f64, t4151: f64, t4248: f64, t508: f64, t5517: f64, t5787: f64, t6983: f64, t7231: f64, t7894: f64, t98615: f64, t98617: f64, t98621: f64, t98623: f64) -> f64 {
    let t101417 = t531 * t28019;
    let t101420 = 6.0_f64 * t2014 * t101417 * t7238;
    let t101422 = 6.0_f64 * t7898 * t25866;
    let t101428 = 12.0_f64 * t28167 * t8996 * t13867;
    let t101431 = 6.0_f64 * t28167 * t8996 * t13872;
    let t101432 = -t98615 - t98617 + t98621 - t98623 - 4.0_f64 * t4248 * t25872 + t2011 * t14310 - t118 * (t100940 + t101120) - t101124 - t101407 * t508 - 2.0_f64 * t28160 * t1310 - t25169 * t1843 - 2.0_f64 * t6983 * t5517 + t101416 + t101420 + t101422 + t7894 * t4151 + 2.0_f64 * t7231 * t5787 + t101428 + t101431;
    t101432
}

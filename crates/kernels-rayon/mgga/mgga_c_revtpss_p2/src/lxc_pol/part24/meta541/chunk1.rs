//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1590/1850 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1590(t25072: f64, t571: f64, t5891: f64, t5915: f64, t5911: f64, t5895: f64, t5823: f64, t5907: f64, t22: f64, t39454: f64, t100: f64, t105: f64, t108: f64, t109: f64, t1507: f64, t1510: f64, t21835: f64, t21860: f64, t22604: f64, t22608: f64, t22618: f64, t22621: f64, t22624: f64, t22625: f64, t22699: f64, t2349: f64, t2357: f64, t4269: f64, t4279: f64, t46196: f64, t46212: f64, t5902: f64, t5908: f64, t5912: f64, t97: f64, tau1: f64) -> (f64, f64, f64, f64, f64) {
    let t86909 = t571 * t25072;
    let t86981 = t5891 * t5891;
    let t86988 = t5915 * t5915;
    let t86994 = t5911 * t5911;
    let t87001 = t5895 * t5895;
    let t87008 = t5823 * t5823;
    let t87021 = t5907 * t5907;
    let t87028 = t22 + t39454;
    let t87029 = 12.0_f64 * t87028;
    let t87046 = 10.0_f64 / 3.0_f64 * t105 * t2357 * t86994 + 40.0_f64 / 9.0_f64 * t105 * t4279 * t22624 + 40.0_f64 / 81.0_f64 * t97 * t46196 * t87001 - 20.0_f64 / 9.0_f64 * t97 * t21835 * t5823 + 10.0_f64 / 3.0_f64 * t97 * t2349 * t87008 + 40.0_f64 / 9.0_f64 * t97 * t4269 * t22604 + 800.0_f64 / 27.0_f64 * t5902 * t5908 + 200.0_f64 / 81.0_f64 * t1507 * t22618 - 200.0_f64 / 9.0_f64 * t1507 * t22621 + 40.0_f64 / 81.0_f64 * t105 * t46212 * t87021 - 20.0_f64 / 9.0_f64 * t105 * t21860 * t5911 + 5.0_f64 / 3.0_f64 * t97 * t100 * t87029 + 6160.0_f64 / 81.0_f64 * tau1 * t22699 * t109 - 8800.0_f64 / 81.0_f64 * t22608 * t1510 + 400.0_f64 / 9.0_f64 * t5902 * t5912 - 100.0_f64 / 9.0_f64 * t1507 * t22625 - 5.0_f64 / 3.0_f64 * t105 * t108 * t87029;
    (t86909, t86981, t86988, t87028, t87046)
}

//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2801/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2801(t22590: f64, t625: f64, t22593: f64, t1513: f64, t5915: f64, t22629: f64, t1504: f64, t5823: f64, t22: f64, t39454: f64, t100: f64, t13475: f64, t2: f64, t21850: f64, t2255: f64, t22596: f64, t22597: f64, t22600: f64, t22604: f64, t22605: f64, t22608: f64, t2349: f64, t4269: f64, t4280: f64, t46196: f64, t49777: f64, t580: f64, t5895: f64, t5902: f64, t656: f64, t658: f64, t662: f64, t97: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t75822 = t625 * t22590;
    let t75831 = t625 * t22593;
    let t75833 = t1513 * t5915;
    let t75843 = t625 * t22629;
    let t75861 = t1504 * t5823;
    let t75879 = 6.0_f64 * t22 + 12.0_f64 * t39454;
    let t75887 = 50.0_f64 / 81.0_f64 * t656 * t22597 + 40.0_f64 / 81.0_f64 * t97 * t46196 * t22596 * t658 - 10.0_f64 / 9.0_f64 * t49777 * t5895 * t2 * t580 - 50.0_f64 / 9.0_f64 * t656 * t22600 - 10.0_f64 / 9.0_f64 * t49777 * t75861 * t658 + 10.0_f64 / 3.0_f64 * t13475 * t2255 * t5823 + 10.0_f64 / 3.0_f64 * t97 * t4269 * t21850 - 25.0_f64 / 9.0_f64 * t656 * t22605 + 10.0_f64 / 9.0_f64 * t97 * t2349 * t22604 * t658 + 5.0_f64 / 3.0_f64 * t97 * t100 * t75879 - 2200.0_f64 / 81.0_f64 * t22608 * t662 + 400.0_f64 / 27.0_f64 * t5902 * t4280;
    (t75822, t75831, t75833, t75843, t75879, t75887)
}

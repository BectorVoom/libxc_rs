//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta309 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1571;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1572;
use chunk2::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1573;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta309(t124: f64, t836: f64, t10779: f64, t2749: f64, t10777: f64, t125: f64, t2722: f64, t2723: f64, t775: f64, t2747: f64, t10730: f64, t10734: f64, t10737: f64, t10742: f64, t10746: f64, t10749: f64, t10752: f64, t10756: f64, t10758: f64, t10762: f64, t10766: f64, t10773: f64, t2730: f64, t2745: f64, t4362: f64, t851: f64, t2645: f64, t4364: f64, t4366: f64, t837: f64, t820: f64, t823: f64, t844: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t10780, t10782, t10783, t10785, t10786) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1571(t124, t836, t10779, t2749, t10777, t125, t2722, t2723, t775);
        let (t10788, t10791) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1572(t10785, t10786, t2747, t10730, t10734, t10737, t10742, t10746, t10749, t10752, t10756, t10758, t10762, t10766, t10773, t10783, t2730, t2745, t4362, t851);
        let (t10794, t10799, t10803, t10807, t10811) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1573(t10785, t2747, t2749, t125, t2645, t4364, t4366, t837, t820, t823, t844);
    (t10780, t10782, t10783, t10786, t10788, t10791, t10794, t10799, t10803, t10807, t10811)
}

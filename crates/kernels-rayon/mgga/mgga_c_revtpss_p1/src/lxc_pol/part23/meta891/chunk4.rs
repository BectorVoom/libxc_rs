//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2845/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2845(t45: f64, t221: f64, t23177: f64, t2484: f64, t2485: f64, t14325: f64, t23216: f64, t1469: f64, t4401: f64, t61303: f64, t14401: f64, t14404: f64, t18272: f64, t18281: f64, t19680: f64, t22671: f64, t22688: f64, t2375: f64, t39825: f64, t4186: f64, t4377: f64, t5825: f64, t606: f64, t76397: f64, t78: f64, zeta_threshold: f64) -> (f64, f64, f64, f64) {
    let t151 = t45 <= zeta_threshold;
    let t76887 = t2484 * t2485 * t221 * t23177;
    let t76890 = 36.0_f64 * t14325 * t23216;
    let t76892 = t4401 * t61303 * t1469;
    let t76893 = 36.0_f64 * t76892;
    let t76911 = piecewise3(t151, 0.0_f64, 40.0_f64 / 81.0_f64 * t39825 * t22688 * t606 - 8.0_f64 / 9.0_f64 * t18272 * t4186 - 8.0_f64 / 9.0_f64 * t14401 * t19680 + 4.0_f64 / 3.0_f64 * t14404 * t5825 + 4.0_f64 / 3.0_f64 * t4377 * t18281 + 4.0_f64 / 9.0_f64 * t2375 * t22671 * t606 + 4.0_f64 / 3.0_f64 * t78 * t76397);
    (t76887, t76890, t76893, t76911)
}

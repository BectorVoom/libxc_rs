//! MGGA_C_REVTPSS lxc pol — lxc_pol part 40 (v4rho3tau_3) CSE chunk 1492/1507 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1492(t1464: f64, t8330: f64, t31204: f64, t575: f64, t1455: f64, t8349: f64, t31244: f64, t571: f64, t2212: f64, t4153: f64, t10199: f64, t2195: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t117153 = t8330 * t1464;
    let t117155 = t31204 * t575;
    let t117161 = t1455 * t8349;
    let t117168 = t571 * t31244;
    let t117170 = t4153 * t2212;
    let t117183 = 154.0_f64 / 27.0_f64 * t10199 * t2195;
    (t117153, t117155, t117161, t117168, t117170, t117183)
}

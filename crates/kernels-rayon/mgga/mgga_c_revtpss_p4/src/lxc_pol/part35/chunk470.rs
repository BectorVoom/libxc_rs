//! MGGA_C_REVTPSS lxc pol — lxc_pol part 35 (v4rho3sigma_10) CSE chunk 470/1234 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part35_v4rho3sigma_10_chunk470(t3153: f64, t3603: f64, t1244: f64, t3598: f64, t3594: f64, t471: f64, t1121: f64, t414: f64, t66: f64, t474: f64, t479: f64, t3089: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t3604 = t3153 * t3603;
    let t3609 = t1244 * t3598;
    let t3610 = t3594 * t3609;
    let t3611 = t3153 * t471;
    let t3617 = 1.0_f64 / t414 / t1121;
    let t3618 = t66 * t3617;
    let t3623 = t474 * t479;
    let t3624 = t3623 * t3089;
    (t3604, t3609, t3610, t3611, t3617, t3618, t3623, t3624)
}

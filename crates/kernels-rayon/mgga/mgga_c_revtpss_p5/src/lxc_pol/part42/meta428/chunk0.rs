//! MGGA_C_REVTPSS lxc pol — lxc_pol part 42 (v4rho3tau_5) CSE chunk 1491/1505 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1491(t1921: f64, t8330: f64, t1913: f64, t8349: f64, t31512: f64, t571: f64, t31463: f64, t575: f64, t1464: f64, t8416: f64, t1455: f64, t8433: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t118091 = 2.0_f64 * t8330 * t1921;
    let t118094 = 2.0_f64 * t1913 * t8349;
    let t118099 = 2.0_f64 * t571 * t31512;
    let t118106 = 2.0_f64 * t31463 * t575;
    let t118108 = 2.0_f64 * t8416 * t1464;
    let t118110 = 2.0_f64 * t1455 * t8433;
    (t118091, t118094, t118099, t118106, t118108, t118110)
}

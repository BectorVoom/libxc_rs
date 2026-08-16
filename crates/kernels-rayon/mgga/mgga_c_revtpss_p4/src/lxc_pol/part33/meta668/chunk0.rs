//! MGGA_C_REVTPSS lxc pol — lxc_pol part 33 (v4rho3sigma_8) CSE chunk 2193/2275 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2193(t27126: f64, t7735: f64, t1497: f64, t4237: f64, t77: f64, t1493: f64, t4241: f64, t5872: f64, t640: f64, t21809: f64, t84: f64, t4186: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t108727 = 4.0_f64 * t27126 * t7735;
    let t108733 = t77 * t4237 * t1497;
    let t108737 = t77 * t1493 * t4241;
    let t108745 = t77 * t640 * t5872;
    let t108749 = t77 * t84 * t21809;
    let t108759 = t77 * t84 * t4186;
    (t108727, t108733, t108737, t108745, t108749, t108759)
}

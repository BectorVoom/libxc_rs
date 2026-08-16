//! MGGA_C_REVTPSS lxc pol — lxc_pol part 39 (v4rho3tau_2) CSE chunk 1120/1507 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1120(t1235: f64, t12984: f64, t12627: f64, t225: f64, t127: f64, t3672: f64, t371: f64, t3671: f64, t140: f64, t3693: f64, t1222: f64, t1226: f64, t697: f64) -> (f64, f64, f64, f64, f64) {
    let t12985 = t1235 * t12984;
    let t12987 = t12627 * t225;
    let t12995 = t371 * t127 * t3672;
    let t12996 = t3671 * t12995;
    let t12998 = t140 * t3693;
    let t12999 = t1222 * t12998;
    let t13011 = t697 * t1226;
    (t12985, t12987, t12996, t12999, t13011)
}

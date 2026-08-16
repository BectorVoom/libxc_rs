//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 1916/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1916(t127: f64, t3672: f64, t371: f64, t3671: f64, t140: f64, t3693: f64, t1222: f64, t1226: f64, t697: f64, t3688: f64, t3700: f64, t3367: f64, t404: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t12995 = t371 * t127 * t3672;
    let t12996 = t3671 * t12995;
    let t12998 = t140 * t3693;
    let t12999 = t1222 * t12998;
    let t13011 = t697 * t1226;
    let t13012 = t1222 * t13011;
    let t13014 = t140 * t3688;
    let t13015 = t1222 * t13014;
    let t13017 = t140 * t3700;
    let t13018 = t1222 * t13017;
    let t13026 = 1.0_f64 / t404 / t3367;
    (t12995, t12996, t12998, t12999, t13011, t13012, t13014, t13015, t13017, t13018, t13026)
}

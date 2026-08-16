//! MGGA_C_REVTPSS lxc pol — lxc_pol part 25 (v4rho3sigma_0) CSE chunk 1083/1360 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part25_v4rho3sigma_0_chunk1083(t1224: f64, t3362: f64, t10356: f64, t1012: f64, t1226: f64, t697: f64, t1222: f64, t140: f64, t3688: f64, t3700: f64, t12268: f64, t3698: f64) -> (f64, f64, f64, f64, f64) {
    let t13006 = t1224 * t3362;
    let t13007 = t13006 * t10356;
    let t13008 = t1012 * t13007;
    let t13011 = t697 * t1226;
    let t13012 = t1222 * t13011;
    let t13014 = t140 * t3688;
    let t13015 = t1222 * t13014;
    let t13017 = t140 * t3700;
    let t13018 = t1222 * t13017;
    let t13020 = t3698 * t12268;
    (t13008, t13012, t13015, t13018, t13020)
}

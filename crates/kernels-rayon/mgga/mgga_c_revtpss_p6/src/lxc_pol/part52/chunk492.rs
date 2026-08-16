//! MGGA_C_REVTPSS lxc pol — lxc_pol part 52 (v4rho2sigma2_7) CSE chunk 492/1292 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part52_v4rho2sigma2_7_chunk492(t1226: f64, t140: f64, t1222: f64, t1121: f64, t404: f64, t1251: f64, t3172: f64, t1247: f64, t1032: f64, t1204: f64, t1246: f64, t1234: f64, t1260: f64) -> (f64, f64, f64, f64, f64) {
    let t3685 = t140 * t1226;
    let t3686 = t1222 * t3685;
    let t3698 = 1.0_f64 / t404 / t1121;
    let t3704 = t3172 * t1251;
    let t3705 = t1247 * t3704;
    let t3707 = t1204 * t1032;
    let t3708 = t3707 * t1246;
    let t3711 = t1234 * t1260;
    (t3686, t3698, t3705, t3708, t3711)
}

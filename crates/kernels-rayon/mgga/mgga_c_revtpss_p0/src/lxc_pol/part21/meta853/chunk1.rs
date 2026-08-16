//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 3213/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3213(t12901: f64, t17572: f64, t17235: f64, t372: f64, t13068: f64, t5292: f64, t1032: f64, t1246: f64, t17331: f64, t1247: f64, t17221: f64, t3172: f64) -> (f64, f64, f64, f64, f64) {
    let t59360 = t17572 * t12901;
    let t59362 = t372 * t17235;
    let t59371 = t13068 * t5292;
    let t59375 = t17331 * t1032 * t1246;
    let t59379 = t1247 * t3172 * t17221;
    (t59360, t59362, t59371, t59375, t59379)
}

//! MGGA_C_REVTPSS lxc pol — lxc_pol part 35 (v4rho3sigma_10) CSE chunk 1165/1234 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part35_v4rho3sigma_10_chunk1165(t102420: f64, t5722: f64, t28780: f64, t98041: f64, t27899: f64, t28845: f64, t28894: f64, t97802: f64, t98380: f64, t97700: f64, t1364: f64, t30248: f64, t786: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t109534 = t102420 * t5722;
    let t109536 = t98041 * t28780;
    let t109539 = t27899 * t28845;
    let t109553 = t97802 * t28894;
    let t109555 = t98380 * t28894;
    let t109567 = t97700 * t28780;
    let t109579 = t786 * t30248 * t1364;
    (t109534, t109536, t109539, t109553, t109555, t109567, t109579)
}

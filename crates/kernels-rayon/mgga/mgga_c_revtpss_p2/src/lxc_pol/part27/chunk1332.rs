//! MGGA_C_REVTPSS lxc pol — lxc_pol part 27 (v4rho3sigma_2) CSE chunk 1332/1333 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part27_v4rho3sigma_2_chunk1332(t2172: f64, t4153: f64, t27110: f64, t571: f64, t13226: f64, t13250: f64, t1456: f64, t1458: f64, t1464: f64, t2168: f64, t27090: f64, t3: f64, t4154: f64, t4168: f64, t575: f64, t7691: f64, t7700: f64, t96684: f64, t96690: f64, t96692: f64, t96694: f64, t97567: f64, t97576: f64) -> f64 {
    let t97580 = t4153 * t2172;
    let t97586 = t571 * t27110;
    let tv4rho3sigma2 = t3 * t575 * t97567 + t13226 * t2172 + t13250 * t2168 + 3.0_f64 * t1456 * t27110 + t1458 * t97576 + 3.0_f64 * t1464 * t27090 + 3.0_f64 * t4154 * t7700 + 3.0_f64 * t4168 * t7691 + 6.0_f64 * t96684 + 6.0_f64 * t96690 + 3.0_f64 * t96692 + 3.0_f64 * t96694 + 3.0_f64 * t97580 + 3.0_f64 * t97586;
    tv4rho3sigma2
}

//! MGGA_C_REVTPSS lxc pol — lxc_pol part 30 (v4rho3sigma_5) CSE chunk 2070/2270 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2070(t3813: f64, t651: f64, t7741: f64, t18153: f64, t1936: f64, t18163: f64, t7742: f64, t28063: f64, t4254: f64, t1937: f64, t75485: f64, t18227: f64, t6993: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t97617 = 2.0_f64 * t651 * t3813 * t7741;
    let t97629 = 2.0_f64 * t651 * t18153 * t1936;
    let t97639 = 2.0_f64 * t18163 * t7742;
    let t97641 = 4.0_f64 * t4254 * t28063;
    let t97643 = 2.0_f64 * t75485 * t1937;
    let t97645 = 4.0_f64 * t18227 * t6993;
    (t97617, t97629, t97639, t97641, t97643, t97645)
}

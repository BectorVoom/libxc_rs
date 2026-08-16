//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 775/1278 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk775(t2095: f64, t8505: f64, t137: f64, t1579: f64, t336: f64, t578: f64, t1494: f64, t2041: f64, t1498: f64, t355: f64, t535: f64, t5720: f64, t599: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t8744 = t2095 * t8505;
    let t8747 = t336 * t1579 * t137;
    let t8748 = t578 * t8747;
    let t8754 = t2041 * t1494;
    let t8756 = t2041 * t1498;
    let t8771 = t535 * t355;
    let t8772 = t2095 * t8771;
    let t8774 = t599 * t5720;
    (t8744, t8747, t8748, t8754, t8756, t8771, t8772, t8774)
}

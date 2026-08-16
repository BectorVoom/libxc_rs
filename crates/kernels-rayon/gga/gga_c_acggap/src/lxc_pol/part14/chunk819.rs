//! GGA_C_ACGGAP lxc pol — lxc_pol part 14 (v4rho3sigma_6) CSE chunk 819/1223 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part14_v4rho3sigma_6_chunk819(t1095: f64, t1426: f64, t9536: f64, t598: f64, t137: f64, t1772: f64, t1083: f64, t1089: f64, t1841: f64, t2118: f64, t2297: f64, t535: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t9538 = t1426 * t1095 * t9536;
    let t9539 = t598 * t9538;
    let t9541 = t137 * t1772;
    let t9543 = t1089 * t1083 * t9541;
    let t9544 = t598 * t9543;
    let t9546 = t2118 * t1841;
    let t9549 = t1426 * t535 * t2297;
    (t9538, t9539, t9541, t9543, t9544, t9546, t9549)
}

//! GGA_C_ACGGAP lxc pol — lxc_pol part 16 (v4rho3sigma_8) CSE chunk 822/1223 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part16_v4rho3sigma_8_chunk822(t1083: f64, t1089: f64, t9563: f64, t598: f64, t1861: f64, t2001: f64, t1851: f64, t3300: f64, t9552: f64, t1095: f64, t4352: f64, t9529: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t9565 = t1089 * t1083 * t9563;
    let t9566 = t598 * t9565;
    let t9568 = t2001 * t1861;
    let t9570 = t2001 * t1851;
    let t9573 = t1089 * t3300 * t9552;
    let t9574 = t598 * t9573;
    let t9577 = t4352 * t1095 * t9529;
    (t9565, t9566, t9568, t9570, t9573, t9574, t9577)
}

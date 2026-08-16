//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 893/1278 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk893(t13716: f64, t577: f64, t584: f64, t1072: f64, t167: f64, t7322: f64, t145: f64, t301: f64, t721: f64, t174: f64, t372: f64, t7859: f64) -> (f64, f64, f64, f64, f64) {
    let t30594 = t13716 * t577;
    let t30595 = t30594 * t584;
    let t30598 = t7322 * t167 * t1072;
    let t30601 = t30598 * t145 * t301 * t721;
    let t30605 = t7859 * t174 * t372 * t721;
    (t30594, t30595, t30598, t30601, t30605)
}

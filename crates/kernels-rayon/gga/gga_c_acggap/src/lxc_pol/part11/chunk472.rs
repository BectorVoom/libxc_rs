//! GGA_C_ACGGAP lxc pol — lxc_pol part 11 (v4rho3sigma_3) CSE chunk 472/1213 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part11_v4rho3sigma_3_chunk472(t105: f64, t2162: f64, t467: f64, t814: f64, t469: f64, t495: f64, t2001: f64, t532: f64, t1501: f64, t336: f64, t570: f64, t525: f64, t599: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t2163 = t105 * t2162;
    let t2166 = t814 * t467;
    let t2254 = t469 * t495;
    let t2258 = t2001 * t532;
    let t2260 = t336 * t1501;
    let t2261 = t570 * t2260;
    let t2263 = t599 * t525;
    (t2163, t2166, t2254, t2258, t2260, t2261, t2263)
}

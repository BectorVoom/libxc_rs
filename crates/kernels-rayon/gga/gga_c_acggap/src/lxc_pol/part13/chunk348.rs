//! GGA_C_ACGGAP lxc pol — lxc_pol part 13 (v4rho3sigma_5) CSE chunk 348/1213 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part13_v4rho3sigma_5_chunk348(t1360: f64, t1394: f64, t150: f64, t153: f64, t94: f64, t420: f64, t495: f64, t301: f64, t1298: f64, t402: f64, t155: f64, t400: f64, t403: f64, t519: f64, t521: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t1396 = (t1360 + t1394) * t150;
    let t1402 = t153 * t94;
    let t1403 = t420 * t495;
    let t1404 = t1403 * t301;
    let t1407 = t402 * t1298;
    let t1410 = -t1396 * t155 - 12.0_f64 * t1402 * t1404 + 3.0_f64 * t1407 * t153 + 3.0_f64 * t400 * t521 + 3.0_f64 * t403 * t519;
    (t1396, t1402, t1403, t1404, t1407, t1410)
}

//! GGA_C_ACGGAP lxc pol — lxc_pol part 14 (v4rho3sigma_6) CSE chunk 325/1223 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part14_v4rho3sigma_6_chunk325(t1049: f64, t503: f64, t1055: f64, t1427: f64, t345: f64, t355: f64, t495: f64) -> (f64, f64, f64, f64) {
    let t1474 = t1049 * t503;
    let t1476 = t1055 * t1427;
    let t1477 = t345 * t1476;
    let t1479 = t355 * t495;
    (t1474, t1476, t1477, t1479)
}

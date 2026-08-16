//! GGA_C_ACGGAP lxc pol — lxc_pol part 13 (v4rho3sigma_5) CSE chunk 978/1213 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part13_v4rho3sigma_5_chunk978(t4099: f64, t469: f64, t1427: f64, t467: f64, t1662: f64, t1679: f64, t2541: f64, t29948: f64, t495: f64, t694: f64, t1298: f64, t7278: f64) -> (f64, f64, f64, f64, f64) {
    let t33393 = t469 * t4099;
    let t33397 = t1427 * t467;
    let t33403 = 2.0_f64 * t1679 * t2541 * t1662;
    let t33409 = 6.0_f64 * t694 * t29948 * t495;
    let t33412 = 6.0_f64 * t694 * t7278 * t1298;
    (t33393, t33397, t33403, t33409, t33412)
}

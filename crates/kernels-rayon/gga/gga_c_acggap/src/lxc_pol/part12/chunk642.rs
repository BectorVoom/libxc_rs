//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 642/1250 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk642(t150: f64, t5019: f64, t5020: f64, t5029: f64, t5047: f64, t400: f64, t94: f64, t1024: f64, t495: f64, t922: f64, t1298: f64, t420: f64) -> (f64, f64, f64, f64) {
    let t5050 = (t5019 + t5020 + t5029 + t5047) * t150;
    let t5060 = t400 * t94;
    let t5065 = t1024 * t495;
    let t5066 = t5065 * t922;
    let t5069 = t420 * t1298;
    (t5050, t5060, t5066, t5069)
}

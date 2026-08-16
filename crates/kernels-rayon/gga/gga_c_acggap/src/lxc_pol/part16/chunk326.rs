//! GGA_C_ACGGAP lxc pol — lxc_pol part 16 (v4rho3sigma_8) CSE chunk 326/1223 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part16_v4rho3sigma_8_chunk326(t1479: f64, t721: f64, t1060: f64, t1298: f64, t346: f64, t345: f64, t1048: f64, t1050: f64, t1054: f64, t1063: f64, t1076: f64, t1474: f64, t1477: f64) -> (f64, f64, f64, f64, f64) {
    let t1480 = t1479 * t721;
    let t1481 = t1060 * t1480;
    let t1483 = t346 * t1298;
    let t1484 = t345 * t1483;
    let t1487 = t1048 + t1050 / 3.0_f64 - t1054 + t1474 / 3.0_f64 + t1477 / 2.0_f64 - t1481 / 24.0_f64 - t1484 / 4.0_f64 - t1063 / 24.0_f64 + t1076;
    (t1480, t1481, t1483, t1484, t1487)
}

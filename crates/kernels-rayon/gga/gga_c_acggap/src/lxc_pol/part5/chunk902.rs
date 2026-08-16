//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 902/1332 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk902(t13502: f64, t425: f64, t431: f64, t438: f64, t1005: f64, t3786: f64, t1032: f64, t3266: f64, t384: f64, t386: f64, t991: f64, t1103: f64, t3770: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t13503 = t13502 * t425;
    let t13505 = t13502 * t431;
    let t13507 = t13502 * t438;
    let t13509 = t1005 * t3786;
    let t13517 = t1032 * t3786;
    let t13521 = t384 * t386 * t3266 * t991;
    let t13532 = t3770 * t1103;
    (t13503, t13505, t13507, t13509, t13517, t13521, t13532)
}

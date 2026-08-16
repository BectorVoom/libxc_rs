//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 916/1332 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk916(t13957: f64, t425: f64, t431: f64, t438: f64, t1200: f64, t3670: f64, t1205: f64, t1032: f64, t3292: f64, t1005: f64, t3732: f64, t3811: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t13958 = t13957 * t425;
    let t13960 = t13957 * t431;
    let t13962 = t13957 * t438;
    let t13964 = t3670 * t1200;
    let t13966 = t3670 * t1205;
    let t13974 = t1032 * t3292;
    let t13985 = t1005 * t3732;
    let t14001 = t1005 * t3811;
    (t13958, t13960, t13962, t13964, t13966, t13974, t13985, t14001)
}

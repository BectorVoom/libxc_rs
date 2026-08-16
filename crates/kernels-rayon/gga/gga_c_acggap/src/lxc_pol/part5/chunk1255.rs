//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1255/1332 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1255(t1111: f64, t1165: f64, t20764: f64, t3391: f64, t1101: f64, t1899: f64, t3361: f64, t1181: f64, t4643: f64, t4718: f64, t4521: f64, t13084: f64, t6343: f64) -> (f64, f64, f64, f64, f64) {
    let t23094 = t3391 * t1165 * t20764 * t1111;
    let t23098 = t3361 * t1165 * t1899 * t1101;
    let t23105 = t3391 * t1181 * t4643 * t4718;
    let t23109 = t3391 * t1181 * t4643 * t4521;
    let t23111 = t13084 * t6343;
    (t23094, t23098, t23105, t23109, t23111)
}

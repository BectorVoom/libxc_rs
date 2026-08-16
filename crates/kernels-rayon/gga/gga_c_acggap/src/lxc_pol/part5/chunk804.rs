//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 804/1332 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk804(t1460: f64, t6337: f64, t1181: f64, t1165: f64, t1884: f64, t407: f64, t1350: f64, t530: f64, t3361: f64, t1539: f64, t5862: f64, t1163: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t6338 = t6337 * t1460;
    let t6339 = t1181 * t6338;
    let t6343 = t1165 * t1884 * t407;
    let t6346 = t530 * t1350;
    let t6347 = t1181 * t6346;
    let t6348 = t3361 * t6347;
    let t6351 = t1165 * t5862 * t1539;
    let t6352 = t1163 * t6351;
    (t6338, t6339, t6343, t6346, t6347, t6348, t6351, t6352)
}

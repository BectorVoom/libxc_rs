//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1025/1332 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1025(t1077: f64, t1163: f64, t1181: f64, t1532: f64, t1539: f64, t1165: f64, t4298: f64, t5127: f64, t1008: f64, t4552: f64, t5260: f64, t3409: f64, t4983: f64) -> (f64, f64, f64, f64, f64) {
    let t17450 = t1163 * t1181 * t1532 * t1539 * t1077;
    let t17454 = t1163 * t1165 * t4298 * t5127;
    let t17468 = t1008 * t4552;
    let t17480 = t1008 * t5260;
    let t17484 = t3409 * t4983;
    (t17450, t17454, t17468, t17480, t17484)
}

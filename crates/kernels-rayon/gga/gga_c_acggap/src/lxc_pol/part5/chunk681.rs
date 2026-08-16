//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 681/1332 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk681(t1084: f64, t1181: f64, t4417: f64, t1111: f64, t1165: f64, t3391: f64, t1008: f64, t1441: f64, t1456: f64, t1462: f64, t1005: f64, t1434: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t4737 = t1181 * t4417 * t1084;
    let t4741 = t1165 * t4417 * t1111;
    let t4742 = t3391 * t4741;
    let t4745 = 0.34299214494455789578e-2_f64 * t1008 * t1441;
    let t4747 = 0.17149607247227894789e-2_f64 * t1008 * t1456;
    let t4748 = t1008 * t1462;
    let t4750 = t1005 * t1434;
    (t4737, t4741, t4742, t4745, t4747, t4748, t4750)
}

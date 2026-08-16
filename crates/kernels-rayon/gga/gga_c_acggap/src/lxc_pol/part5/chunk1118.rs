//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1118/1332 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1118(t15016: f64, t15018: f64, t11841: f64, t11843: f64, t11849: f64, t6021: f64, t912: f64, t1: f64, t283: f64, t5474: f64, t1708: f64, t40: f64, t803: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t20007 = 160.0_f64 * t15016;
    let t20008 = 240.0_f64 * t15018;
    let t20009 = 24.0_f64 * t11841;
    let t20010 = 240.0_f64 * t11843;
    let t20011 = 2.0_f64 * t11849;
    let t20012 = t6021 * t912;
    let t20013 = 0.11696447245269292414e1_f64 * t20012;
    let t20015 = t5474 * t1 * t283;
    let t20016 = 0.36622894612013090108e-3_f64 * t20015;
    let t20018 = t40 * t1708 * t803;
    (t20007, t20008, t20009, t20010, t20011, t20013, t20016, t20018)
}

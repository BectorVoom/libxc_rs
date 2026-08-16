//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 579/1278 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk579(t1539: f64, t360: f64, t1181: f64, t1532: f64, t1163: f64, t372: f64, t1165: f64, t1552: f64, t4210: f64, t4241: f64, t3456: f64, t1315: f64, t3621: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t5122 = t1539 * t360;
    let t5124 = t1181 * t1532 * t5122;
    let t5126 = 0.85748036236139473944e-3_f64 * t1163 * t5124;
    let t5127 = t1539 * t372;
    let t5129 = t1165 * t1552 * t5127;
    let t5131 = 0.85748036236139473944e-3_f64 * t1163 * t5129;
    let t5133 = t1165 * t1532 * t4210;
    let t5135 = 0.42874018118069736972e-3_f64 * t1163 * t5133;
    let t5147 = t1165 * t1532 * t4241;
    let t5149 = 0.85748036236139473944e-3_f64 * t3456 * t5147;
    let t5169 = 7.0_f64 / 24.0_f64 * t3621 * t1315;
    (t5122, t5124, t5126, t5127, t5129, t5131, t5133, t5135, t5147, t5149, t5169)
}

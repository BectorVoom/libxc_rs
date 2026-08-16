//! GGA_C_ACGGAP lxc pol — lxc_pol part 13 (v4rho3sigma_5) CSE chunk 649/1213 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part13_v4rho3sigma_5_chunk649(t1539: f64, t372: f64, t1165: f64, t1552: f64, t1163: f64, t1532: f64, t4210: f64, t1533: f64, t360: f64, t1181: f64, t4241: f64, t3456: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t5127 = t1539 * t372;
    let t5129 = t1165 * t1552 * t5127;
    let t5131 = 0.85748036236139473944e-3_f64 * t1163 * t5129;
    let t5133 = t1165 * t1532 * t4210;
    let t5135 = 0.42874018118069736972e-3_f64 * t1163 * t5133;
    let t5136 = t1533 * t360;
    let t5138 = t1181 * t1532 * t5136;
    let t5141 = t1533 * t372;
    let t5143 = t1165 * t1552 * t5141;
    let t5147 = t1165 * t1532 * t4241;
    let t5149 = 0.85748036236139473944e-3_f64 * t3456 * t5147;
    (t5127, t5129, t5131, t5133, t5135, t5136, t5138, t5141, t5143, t5147, t5149)
}

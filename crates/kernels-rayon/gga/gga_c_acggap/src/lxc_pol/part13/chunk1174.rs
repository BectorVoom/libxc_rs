//! GGA_C_ACGGAP lxc pol — lxc_pol part 13 (v4rho3sigma_5) CSE chunk 1174/1213 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part13_v4rho3sigma_5_chunk1174(t1181: f64, t4623: f64, t604: f64, t7426: f64, t30090: f64, t8897: f64, t31362: f64, t8903: f64, t7839: f64, t8908: f64, t8912: f64, t1165: f64, t2068: f64, t35102: f64, t7351: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t36081 = t7426 * t1181 * t604 * t4623;
    let t36082 = 0.62896184579208304136e-3_f64 * t36081;
    let t36083 = t30090 * t8897;
    let t36085 = t31362 * t8903;
    let t36086 = 0.10718504529517434243e-2_f64 * t36085;
    let t36087 = t7839 * t8908;
    let t36088 = 0.42874018118069736972e-3_f64 * t36087;
    let t36089 = t7839 * t8912;
    let t36090 = 0.21437009059034868486e-3_f64 * t36089;
    let t36093 = t2068 * t1165 * t7351 * t35102;
    (t36082, t36083, t36086, t36088, t36090, t36093)
}

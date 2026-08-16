//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 737/1336 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk737(t1626: f64, t501: f64, t1662: f64, t496: f64, t572: f64, t81: f64, t79: f64, t127: f64, t4803: f64, t500: f64, t78: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t5130 = 12.0_f64 * t501 * t1626;
    let t5131 = t496 * t1662;
    let t5132 = 12.0_f64 * t5131;
    let t5133 = t501 * t1662;
    let t5134 = 12.0_f64 * t5133;
    let t5135 = t81 * t572;
    let t5136 = 1.0_f64 / t5135;
    let t5137 = t79 * t5136;
    let t5139 = 120.0_f64 * t5137 * t127;
    let t5141 = 24.0_f64 * t4803 * t127;
    let t5142 = t78 * t500;
    let t5143 = t5142 * t127;
    (t5130, t5131, t5132, t5133, t5134, t5135, t5136, t5137, t5139, t5141, t5142, t5143)
}

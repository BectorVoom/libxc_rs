//! GGA_C_ACGGAP lxc pol — lxc_pol part 16 (v4rho3sigma_8) CSE chunk 976/1223 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part16_v4rho3sigma_8_chunk976(t34534: f64, t5133: f64, t7647: f64, t5101: f64, t7741: f64, t1434: f64, t7746: f64, t4680: f64, t7426: f64, t8476: f64, t30937: f64, t8450: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t34535 = 0.17149607247227894789e-2_f64 * t34534;
    let t34537 = t7647 * t5133;
    let t34538 = 0.85748036236139473944e-3_f64 * t34537;
    let t34547 = t7741 * t5101;
    let t34549 = t7746 * t1434;
    let t34556 = t7426 * t4680 * t8476;
    let t34557 = 0.62896184579208304136e-3_f64 * t34556;
    let t34561 = t30937 * t8450;
    (t34535, t34538, t34547, t34549, t34557, t34561)
}

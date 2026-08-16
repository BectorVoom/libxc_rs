//! GGA_C_ACGGAP lxc pol — lxc_pol part 13 (v4rho3sigma_5) CSE chunk 1058/1213 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part13_v4rho3sigma_5_chunk1058(t5129: f64, t7647: f64, t5133: f64, t2001: f64, t4518: f64, t4667: f64, t5267: f64, t5096: f64, t5101: f64, t7741: f64, t1434: f64, t7746: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t34534 = t7647 * t5129;
    let t34535 = 0.17149607247227894789e-2_f64 * t34534;
    let t34537 = t7647 * t5133;
    let t34538 = 0.85748036236139473944e-3_f64 * t34537;
    let t34539 = t2001 * t4518;
    let t34541 = t2001 * t4667;
    let t34543 = t2001 * t5267;
    let t34545 = t2001 * t5096;
    let t34547 = t7741 * t5101;
    let t34549 = t7746 * t1434;
    (t34535, t34538, t34539, t34541, t34543, t34545, t34547, t34549)
}

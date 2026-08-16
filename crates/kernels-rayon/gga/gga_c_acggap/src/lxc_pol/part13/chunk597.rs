//! GGA_C_ACGGAP lxc pol — lxc_pol part 13 (v4rho3sigma_5) CSE chunk 597/1213 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part13_v4rho3sigma_5_chunk597(t1096: f64, t1165: f64, t4417: f64, t1466: f64, t3409: f64, t1106: f64, t1181: f64, t540: f64, t3391: f64, t1131: f64, t336: f64, t535: f64) -> (f64, f64, f64, f64, f64) {
    let t4419 = t1165 * t4417 * t1096;
    let t4423 = 0.40015750243531754508e-2_f64 * t3409 * t1466;
    let t4425 = t1181 * t540 * t1106;
    let t4427 = 0.17149607247227894789e-2_f64 * t3391 * t4425;
    let t4430 = t336 * t535 * t1131;
    (t4419, t4423, t4425, t4427, t4430)
}

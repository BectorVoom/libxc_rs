//! GGA_C_ACGGAP lxc pol — lxc_pol part 14 (v4rho3sigma_6) CSE chunk 582/1223 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part14_v4rho3sigma_6_chunk582(t1089: f64, t175: f64, t5249: f64, t384: f64, t1032: f64, t1423: f64, t1539: f64, t301: f64, t1165: f64, t1532: f64, t3194: f64, t1647: f64, t879: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t5251 = t1089 * t175 * t5249;
    let t5253 = 0.17149607247227894789e-2_f64 * t384 * t5251;
    let t5263 = t1032 * t1423;
    let t5284 = t1539 * t301;
    let t5286 = t1165 * t1532 * t5284;
    let t5288 = 0.17149607247227894789e-2_f64 * t3194 * t5286;
    let t5304 = t1647 * t879;
    (t5251, t5253, t5263, t5284, t5286, t5288, t5304)
}

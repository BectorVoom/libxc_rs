//! GGA_C_ACGGAP lxc pol — lxc_pol part 11 (v4rho3sigma_3) CSE chunk 656/1213 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part11_v4rho3sigma_3_chunk656(t513: f64, t922: f64, t1095: f64, t1426: f64, t1175: f64, t360: f64, t1181: f64, t1532: f64, t372: f64, t1165: f64, t1552: f64, t3196: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t5265 = t513 * t922;
    let t5267 = t1426 * t1095 * t5265;
    let t5270 = t1175 * t360;
    let t5272 = t1181 * t1532 * t5270;
    let t5275 = t1175 * t372;
    let t5277 = t1165 * t1552 * t5275;
    let t5281 = t1165 * t1532 * t3196;
    (t5265, t5267, t5270, t5272, t5275, t5277, t5281)
}

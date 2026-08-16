//! GGA_C_ACGGAP lxc pol — lxc_pol part 13 (v4rho3sigma_5) CSE chunk 589/1213 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part13_v4rho3sigma_5_chunk589(t1101: f64, t1165: f64, t530: f64, t4282: f64, t1470: f64, t3409: f64, t1410: f64, t174: f64, t1175: f64, t1181: f64, t1182: f64, t435: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t4284 = t1165 * t530 * t1101;
    let t4285 = t4282 * t4284;
    let t4288 = 0.40015750243531754508e-2_f64 * t3409 * t1470;
    let t4289 = t174 * t1410;
    let t4291 = t1165 * t4289 * t1175;
    let t4295 = t1181 * t4289 * t1182;
    let t4298 = t435 * t1410;
    (t4284, t4285, t4288, t4289, t4291, t4295, t4298)
}

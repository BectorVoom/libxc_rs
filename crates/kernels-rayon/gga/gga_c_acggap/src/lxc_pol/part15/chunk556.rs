//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 556/1278 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk556(t1106: f64, t1181: f64, t530: f64, t3361: f64, t1111: f64, t1165: f64, t4267: f64, t1562: f64, t3431: f64, t3360: f64, t3402: f64, t1101: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t4273 = t1181 * t530 * t1106;
    let t4275 = 0.34299214494455789578e-2_f64 * t3361 * t4273;
    let t4277 = t1165 * t4267 * t1111;
    let t4279 = 0.34299214494455789578e-2_f64 * t3361 * t4277;
    let t4280 = t3431 * t1562;
    let t4282 = t3360 * t3402;
    let t4284 = t1165 * t530 * t1101;
    (t4273, t4275, t4277, t4279, t4280, t4282, t4284)
}

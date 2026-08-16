//! MGGA_C_REVTPSS lxc pol — lxc_pol part 27 (v4rho3sigma_2) CSE chunk 791/1333 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part27_v4rho3sigma_2_chunk791(t9484: f64, t9543: f64, t520: f64, t512: f64, t1450: f64, t4135: f64, t177: f64, t3850: f64, t762: f64, t749: f64, t1353: f64, t198: f64, t4139: f64, t566: f64, t9399: f64, t9400: f64, t9405: f64, t9407: f64, t9409: f64, t9412: f64, t9415: f64, t9421: f64, t9423: f64, t9427: f64, t9430: f64) -> (f64, f64, f64, f64, f64) {
    let t9544 = t9484 + t9543;
    let t9545 = t520 * t9544;
    let t9546 = t512 * t9545;
    let t9547 = t4135 * t1450;
    let t9551 = t3850 * t177;
    let t9552 = t9551 * t762;
    let t9553 = 0.17544670867903938621e1_f64 * t9552;
    let t9554 = t3850 * t749;
    let t9555 = t512 * t9554;
    let t9556 = 3.0_f64 * t9555;
    let t9557 = 9.0_f64 * t1353 * t4139 * t9547 + 6.0_f64 * t198 * t566 * t9400 - t9399 + t9405 + t9407 - t9409 + t9412 - t9415 + t9421 + t9423 - t9427 + t9430 + t9546 - t9553 + t9556;
    (t9544, t9546, t9553, t9556, t9557)
}

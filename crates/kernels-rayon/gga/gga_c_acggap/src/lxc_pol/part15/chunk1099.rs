//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 1099/1278 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk1099(t24196: f64, t336: f64, t570: f64, t1181: f64, t38766: f64, t604: f64, t7413: f64, t38771: f64, t599: f64, t5618: f64, t7561: f64, t1165: f64, t25941: f64, t7337: f64) -> (f64, f64, f64, f64, f64) {
    let t38986 = t570 * t336 * t24196;
    let t38990 = t7413 * t1181 * t604 * t38766;
    let t38994 = t7413 * t1181 * t599 * t38771;
    let t38996 = t7561 * t5618;
    let t39000 = t7337 * t1165 * t604 * t25941;
    (t38986, t38990, t38994, t38996, t39000)
}

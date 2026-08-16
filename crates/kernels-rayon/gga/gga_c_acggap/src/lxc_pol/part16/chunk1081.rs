//! GGA_C_ACGGAP lxc pol — lxc_pol part 16 (v4rho3sigma_8) CSE chunk 1081/1223 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part16_v4rho3sigma_8_chunk1081(t1181: f64, t38766: f64, t604: f64, t7413: f64, t38771: f64, t599: f64, t5618: f64, t7561: f64, t1165: f64, t25941: f64, t7337: f64, t31428: f64, t9614: f64) -> (f64, f64, f64, f64, f64) {
    let t38990 = t7413 * t1181 * t604 * t38766;
    let t38994 = t7413 * t1181 * t599 * t38771;
    let t38996 = t7561 * t5618;
    let t39000 = t7337 * t1165 * t604 * t25941;
    let t39002 = t31428 * t9614;
    (t38990, t38994, t38996, t39000, t39002)
}

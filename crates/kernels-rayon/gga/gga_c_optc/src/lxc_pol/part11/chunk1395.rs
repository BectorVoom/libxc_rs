//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1395/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1395(t17356: f64, t4299: f64, t4300: f64, t277: f64, t2911: f64, t3245: f64, t34301: f64, t34309: f64, t4281: f64, t4282: f64, t4289: f64, t4290: f64, t4297: f64, t53769: f64, t53776: f64, t58827: f64, t58834: f64, t58836: f64, t58864: f64, t58865: f64, t95: f64) -> f64 {
    let t58875 = t4299 * t4300 * t17356;
    let t58878 = -2464.0_f64 / 81.0_f64 * t53769 + 20.0_f64 / 81.0_f64 * t34301 + 20.0_f64 / 27.0_f64 * t34309 - 0.77534644304710291488e-2_f64 * t95 * t277 * t58827 * t2911 + 80000.0_f64 / 81.0_f64 * t53776 + t58834 + t58836 + t58864 - 4.0_f64 / 3.0_f64 * t4281 * t3245 * t4282 * t58865 + 8.0_f64 / 9.0_f64 * t4281 * t4289 * t4290 * t58865 + 200.0_f64 / 81.0_f64 * t4297 * t58875;
    t58878
}

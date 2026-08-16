//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 1126/1278 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk1126(t1318: f64, t507: f64, t7436: f64, t142: f64, t6388: f64, t8806: f64, t5906: f64, t1181: f64, t5693: f64, t604: f64, t8463: f64, t5697: f64, t7351: f64, t7575: f64) -> (f64, f64, f64, f64, f64) {
    let t39471 = t7436 * t507 * t1318;
    let t39474 = t8806 * t142 * t6388;
    let t39477 = t7436 * t142 * t5906;
    let t39485 = t8463 * t1181 * t604 * t5693;
    let t39489 = t7575 * t1181 * t7351 * t5697;
    (t39471, t39474, t39477, t39485, t39489)
}

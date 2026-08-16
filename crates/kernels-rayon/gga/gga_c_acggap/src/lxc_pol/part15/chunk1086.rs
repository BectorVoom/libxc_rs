//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 1086/1278 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk1086(t1165: f64, t38766: f64, t7351: f64, t7413: f64, t1410: f64, t8790: f64, t604: f64, t1815: f64, t322: f64, t1181: f64, t31562: f64, t599: f64) -> (f64, f64, f64, f64, f64) {
    let t38769 = t7413 * t1165 * t7351 * t38766;
    let t38771 = t8790 * t1410;
    let t38774 = t7413 * t1165 * t604 * t38771;
    let t38778 = t1815 * t322;
    let t38781 = t31562 * t1181 * t599 * t38778;
    (t38769, t38771, t38774, t38778, t38781)
}

//! GGA_C_ACGGAP lxc pol — lxc_pol part 14 (v4rho3sigma_6) CSE chunk 1118/1223 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part14_v4rho3sigma_6_chunk1118(t1181: f64, t5693: f64, t604: f64, t8463: f64, t5697: f64, t7351: f64, t7575: f64, t1849: f64, t322: f64, t1165: f64, t7493: f64, t5608: f64, t7561: f64) -> (f64, f64, f64, f64, f64) {
    let t39485 = t8463 * t1181 * t604 * t5693;
    let t39489 = t7575 * t1181 * t7351 * t5697;
    let t39491 = t1849 * t322;
    let t39494 = t7493 * t1165 * t7351 * t39491;
    let t39497 = t7561 * t5608;
    (t39485, t39489, t39491, t39494, t39497)
}

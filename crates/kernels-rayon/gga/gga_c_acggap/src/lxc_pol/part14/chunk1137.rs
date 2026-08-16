//! GGA_C_ACGGAP lxc pol — lxc_pol part 14 (v4rho3sigma_6) CSE chunk 1137/1223 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part14_v4rho3sigma_6_chunk1137(t1181: f64, t1849: f64, t360: f64, t7351: f64, t7575: f64, t604: f64, t6209: f64, t2060: f64, t372: f64, t8927: f64, t9563: f64, t5694: f64, t8806: f64) -> (f64, f64, f64, f64) {
    let t39720 = t7575 * t1181 * t7351 * t1849 * t360;
    let t39724 = t7575 * t1181 * t604 * t6209;
    let t39733 = t2060 * t8927 * t9563 * t372;
    let t39735 = t8806 * t5694;
    (t39720, t39724, t39733, t39735)
}

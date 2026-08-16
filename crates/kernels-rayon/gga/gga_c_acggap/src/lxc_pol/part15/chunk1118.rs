//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 1118/1278 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk1118(t7447: f64, t9663: f64, t7440: f64, t9734: f64, t31773: f64, t9660: f64, t9730: f64, t2030: f64, t361: f64, t9700: f64, t142: f64, t5506: f64, t599: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t39320 = t7447 * t9663;
    let t39322 = t7440 * t9734;
    let t39324 = t31773 * t9660;
    let t39326 = t7447 * t9730;
    let t39330 = t2030 * t361 * t9700;
    let t39334 = t2030 * t142 * t599 * t5506;
    (t39320, t39322, t39324, t39326, t39330, t39334)
}

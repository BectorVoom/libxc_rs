//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 3310/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3310(t23087: f64, t47672: f64, t1448: f64, t1907: f64, t22483: f64, t22496: f64, t27153: f64, t28198: f64, t4139: f64, t47088: f64, t47092: f64, t47096: f64, t47098: f64, t5541: f64, t73407: f64, t73499: f64, t85976: f64, t85979: f64) -> f64 {
    let t86791 = t23087 * t47672;
    let t86804 = -6.0_f64 * t1448 * t5541 * t86791 - 3.0_f64 * t1907 * t5541 * t73407 - 9.0_f64 * t22483 * t22496 * t4139 - 9.0_f64 * t22483 * t27153 * t4139 + 6.0_f64 * t28198 * t5541 * t73499 + t47088 + t47092 - t47096 - t47098 + t85976 + t85979;
    t86804
}

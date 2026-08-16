//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 1351/1383 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk1351(t65624: f64, t65634: f64, t65647: f64, t65650: f64, t67175: f64, t67183: f64, t67185: f64, t69551: f64, t69553: f64, t69555: f64, t69558: f64, t69561: f64, t69564: f64) -> f64 {
    let t71807 = -t69551 / 768.0_f64 - 35.0_f64 / 288.0_f64 * t69553 + 7.0_f64 / 288.0_f64 * t69555 - 119.0_f64 / 1728.0_f64 * t65624 + t67175 - t65634 - t69558 / 192.0_f64 - t69561 / 2.0_f64 + t69564 / 4.0_f64 - t67183 + t67185 - 119.0_f64 / 432.0_f64 * t65647 - t65650;
    t71807
}

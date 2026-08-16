//! GGA_C_ACGGAP lxc pol — lxc_pol part 16 (v4rho3sigma_8) CSE chunk 1125/1223 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part16_v4rho3sigma_8_chunk1125(t1181: f64, t26459: f64, t599: f64, t7337: f64, t1983: f64, t30692: f64, t7586: f64, t9587: f64, t30689: f64, t5732: f64, t1815: f64, t360: f64, t604: f64, t7413: f64) -> (f64, f64, f64, f64) {
    let t39581 = t7337 * t1181 * t599 * t26459;
    let t39585 = t30692 * t7586 * t1983 * t9587;
    let t39587 = t30689 * t5732;
    let t39592 = t7413 * t1181 * t604 * t1815 * t360;
    (t39581, t39585, t39587, t39592)
}

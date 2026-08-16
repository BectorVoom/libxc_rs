//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 1131/1278 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk1131(t4680: f64, t7337: f64, t9588: f64, t1181: f64, t26995: f64, t599: f64, t26459: f64, t1983: f64, t30692: f64, t7586: f64, t9587: f64, t30689: f64, t5732: f64) -> (f64, f64, f64, f64, f64) {
    let t39570 = t7337 * t4680 * t9588;
    let t39574 = t7337 * t1181 * t599 * t26995;
    let t39581 = t7337 * t1181 * t599 * t26459;
    let t39585 = t30692 * t7586 * t1983 * t9587;
    let t39587 = t30689 * t5732;
    (t39570, t39574, t39581, t39585, t39587)
}

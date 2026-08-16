//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 1106/1278 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk1106(t7433: f64, t9641: f64, t1851: f64, t7614: f64, t1967: f64, t9681: f64, t1988: f64, t9531: f64, t429: f64, t4352: f64, t598: f64, t9529: f64) -> (f64, f64, f64, f64, f64) {
    let t39100 = t7433 * t9641;
    let t39107 = t7614 * t1851;
    let t39112 = t1967 * t9681;
    let t39114 = t1988 * t9531;
    let t39118 = t598 * t4352 * t429 * t9529;
    (t39100, t39107, t39112, t39114, t39118)
}

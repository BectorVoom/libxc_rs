//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 1087/1278 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk1087(t157: f64, t1838: f64, t406: f64, t1165: f64, t2068: f64, t604: f64, t1815: f64, t301: f64, t1181: f64, t30698: f64, t599: f64, t1479: f64, t535: f64) -> (f64, f64, f64, f64, f64) {
    let t38784 = t1838 * t406 * t157;
    let t38787 = t2068 * t1165 * t604 * t38784;
    let t38789 = t1815 * t301;
    let t38792 = t30698 * t1181 * t599 * t38789;
    let t38795 = t535 * t1479;
    (t38784, t38787, t38789, t38792, t38795)
}

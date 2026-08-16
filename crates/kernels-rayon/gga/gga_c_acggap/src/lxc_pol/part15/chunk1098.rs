//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 1098/1278 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk1098(t1165: f64, t5544: f64, t7351: f64, t7575: f64, t1849: f64, t30148: f64, t30154: f64, t7842: f64, t30937: f64, t9608: f64, t1181: f64, t5527: f64, t7564: f64, t8600: f64) -> (f64, f64, f64, f64) {
    let t38968 = t7575 * t1165 * t7351 * t5544;
    let t38976 = t30154 * t7842 * t30148 * t1849;
    let t38978 = t30937 * t9608;
    let t38982 = t7564 * t1181 * t8600 * t5527;
    (t38968, t38976, t38978, t38982)
}

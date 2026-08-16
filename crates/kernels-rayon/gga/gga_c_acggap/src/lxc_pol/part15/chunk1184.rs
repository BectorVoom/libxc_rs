//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 1184/1278 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk1184(t1181: f64, t604: f64, t6218: f64, t7575: f64, t6198: f64, t7351: f64, t7564: f64, t1881: f64, t7614: f64, t17912: f64, t2302: f64, t31443: f64, t8906: f64) -> (f64, f64, f64, f64) {
    let t40501 = t7575 * t1181 * t604 * t6218;
    let t40505 = t7564 * t1181 * t7351 * t6198;
    let t40507 = t7614 * t1881;
    let t40511 = t31443 * t17912 * t2302 * t8906;
    (t40501, t40505, t40507, t40511)
}

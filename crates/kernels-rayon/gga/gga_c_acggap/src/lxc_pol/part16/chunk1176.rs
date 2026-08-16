//! GGA_C_ACGGAP lxc pol — lxc_pol part 16 (v4rho3sigma_8) CSE chunk 1176/1223 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part16_v4rho3sigma_8_chunk1176(t1181: f64, t30282: f64, t38843: f64, t599: f64, t1814: f64, t2937: f64, t406: f64, t1165: f64, t30856: f64, t604: f64, t31362: f64, t9597: f64) -> (f64, f64, f64, f64) {
    let t40212 = t30282 * t1181 * t599 * t38843;
    let t40215 = t1814 * t2937 * t406;
    let t40218 = t30856 * t1165 * t604 * t40215;
    let t40220 = t31362 * t9597;
    (t40212, t40215, t40218, t40220)
}

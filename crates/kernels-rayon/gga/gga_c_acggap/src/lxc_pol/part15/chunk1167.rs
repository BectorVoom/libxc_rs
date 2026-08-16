//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 1167/1278 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk1167(t2068: f64, t2069: f64, t40206: f64, t1181: f64, t30282: f64, t38843: f64, t599: f64, t1814: f64, t2937: f64, t406: f64, t1165: f64, t30856: f64, t604: f64) -> (f64, f64, f64, f64) {
    let t40208 = t2068 * t40206 * t2069;
    let t40212 = t30282 * t1181 * t599 * t38843;
    let t40215 = t1814 * t2937 * t406;
    let t40218 = t30856 * t1165 * t604 * t40215;
    (t40208, t40212, t40215, t40218)
}

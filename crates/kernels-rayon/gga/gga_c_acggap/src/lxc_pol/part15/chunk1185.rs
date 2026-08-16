//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 1185/1278 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk1185(t13287: f64, t2302: f64, t31443: f64, t8402: f64, t2001: f64, t5956: f64, t5961: f64, t6205: f64, t6211: f64, t2118: f64, t6215: f64, t1998: f64, t6194: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t40515 = t31443 * t13287 * t2302 * t8402;
    let t40517 = t2001 * t5956;
    let t40519 = t2001 * t5961;
    let t40521 = t2001 * t6205;
    let t40523 = t2001 * t6211;
    let t40525 = t2118 * t6215;
    let t40527 = t1998 * t6194;
    (t40515, t40517, t40519, t40521, t40523, t40525, t40527)
}

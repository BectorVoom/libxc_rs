//! GGA_C_ACGGAP lxc pol — lxc_pol part 14 (v4rho3sigma_6) CSE chunk 1198/1223 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part14_v4rho3sigma_6_chunk1198(t1881: f64, t7614: f64, t17912: f64, t2302: f64, t31443: f64, t8906: f64, t13287: f64, t8402: f64, t2001: f64, t5956: f64, t5961: f64, t6205: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t40507 = t7614 * t1881;
    let t40511 = t31443 * t17912 * t2302 * t8906;
    let t40515 = t31443 * t13287 * t2302 * t8402;
    let t40517 = t2001 * t5956;
    let t40519 = t2001 * t5961;
    let t40521 = t2001 * t6205;
    (t40507, t40511, t40515, t40517, t40519, t40521)
}

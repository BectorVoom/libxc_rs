//! GGA_C_ACGGAP lxc pol — lxc_pol part 14 (v4rho3sigma_6) CSE chunk 631/1223 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part14_v4rho3sigma_6_chunk631(t150: f64, t6004: f64, t6010: f64, t6019: f64, t6036: f64, t519: f64, t94: f64, t1024: f64, t1713: f64, t301: f64, t1298: f64, t1403: f64) -> (f64, f64, f64, f64) {
    let t6039 = (t6004 + t6010 + t6019 + t6036) * t150;
    let t6045 = t519 * t94;
    let t6052 = t1024 * t1713;
    let t6053 = t6052 * t301;
    let t6056 = t1403 * t1298;
    (t6039, t6045, t6053, t6056)
}

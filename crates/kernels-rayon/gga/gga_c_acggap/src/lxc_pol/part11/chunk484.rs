//! GGA_C_ACGGAP lxc pol — lxc_pol part 11 (v4rho3sigma_3) CSE chunk 484/1213 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part11_v4rho3sigma_3_chunk484(t2304: f64, t598: f64, t2118: f64, t527: f64, t1426: f64, t2297: f64, t368: f64) -> (f64, f64, f64) {
    let t2305 = t598 * t2304;
    let t2307 = t2118 * t527;
    let t2310 = t1426 * t368 * t2297;
    (t2305, t2307, t2310)
}

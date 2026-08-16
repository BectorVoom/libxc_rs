//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 439/1404 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk439(t1791: f64, t213: f64, t582: f64, t618: f64, t616: f64, t196: f64, t596: f64) -> (f64, f64, f64, f64) {
    let t1792 = t213 * t1791;
    let t1798 = t582 * t618;
    let t1799 = t616 * t1798;
    let t1802 = 1.0_f64 / t596 / t196;
    (t1792, t1798, t1799, t1802)
}

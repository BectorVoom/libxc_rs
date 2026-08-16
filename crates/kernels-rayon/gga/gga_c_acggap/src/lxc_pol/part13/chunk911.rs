//! GGA_C_ACGGAP lxc pol — lxc_pol part 13 (v4rho3sigma_5) CSE chunk 911/1213 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part13_v4rho3sigma_5_chunk911(t2113: f64, t7630: f64, t30546: f64, t7499: f64, t30543: f64, t7867: f64, t2450: f64, t7432: f64) -> (f64, f64, f64, f64) {
    let t30926 = t7630 * t2113;
    let t30928 = t30546 * t7499;
    let t30932 = t30543 * t7867;
    let t30934 = t2450 * t7432;
    (t30926, t30928, t30932, t30934)
}

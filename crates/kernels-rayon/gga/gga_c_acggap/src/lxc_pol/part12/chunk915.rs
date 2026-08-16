//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 915/1250 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk915(t1200: f64, t7605: f64, t1988: f64, t7535: f64, t30589: f64, t7548: f64, t2109: f64, t7630: f64, t2113: f64, t30546: f64, t7499: f64, t30543: f64, t7867: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t30916 = t7605 * t1200;
    let t30918 = t1988 * t7535;
    let t30920 = t30589 * t7548;
    let t30924 = t7630 * t2109;
    let t30926 = t7630 * t2113;
    let t30928 = t30546 * t7499;
    let t30932 = t30543 * t7867;
    (t30916, t30918, t30920, t30924, t30926, t30928, t30932)
}

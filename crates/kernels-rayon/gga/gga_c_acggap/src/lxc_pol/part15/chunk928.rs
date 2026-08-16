//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 928/1278 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk928(t2113: f64, t7610: f64, t2082: f64, t30567: f64, t7528: f64, t7637: f64, t2109: f64, t1101: f64, t1983: f64, t30827: f64, t7586: f64, t3378: f64, t7584: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t31660 = t7610 * t2113;
    let t31662 = t30567 * t2082;
    let t31682 = t7637 * t7528;
    let t31684 = t7610 * t2109;
    let t31693 = t30827 * t7586 * t1983 * t1101;
    let t31699 = t3378 * t7584;
    (t31660, t31662, t31682, t31684, t31693, t31699)
}

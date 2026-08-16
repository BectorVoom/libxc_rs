//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 1027/1278 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk1027(t30364: f64, t5147: f64, t1992: f64, t30692: f64, t7842: f64, t8901: f64, t30689: f64, t4967: f64, t525: f64, t864: f64, t1165: f64, t31567: f64, t604: f64) -> (f64, f64, f64, f64, f64) {
    let t36006 = t30364 * t5147;
    let t36010 = t30692 * t7842 * t1992 * t8901;
    let t36017 = t30689 * t4967;
    let t36019 = t525 * t864;
    let t36022 = t31567 * t1165 * t604 * t36019;
    (t36006, t36010, t36017, t36019, t36022)
}

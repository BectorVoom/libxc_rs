//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 1163/1278 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk1163(t2001: f64, t5539: f64, t31346: f64, t6328: f64, t6140: f64, t5891: f64, t7561: f64, t1809: f64, t2020: f64, t422: f64, t5784: f64, t598: f64, t599: f64, t6: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t40131 = t2001 * t5539;
    let t40134 = t31346 * t6328;
    let t40136 = t31346 * t6140;
    let t40145 = t7561 * t5891;
    let t40147 = t2020 * t1809;
    let t40152 = t598 * t422 * t6 * t5784 * t599;
    (t40131, t40134, t40136, t40145, t40147, t40152)
}

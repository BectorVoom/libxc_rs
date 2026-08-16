//! GGA_C_ACGGAP lxc pol — lxc_pol part 11 (v4rho3sigma_3) CSE chunk 557/1213 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part11_v4rho3sigma_3_chunk557(t1089: f64, t175: f64, t3809: f64, t384: f64, t1080: f64, t330: f64, t363: f64, t987: f64, t3243: f64, t453: f64, t1210: f64, t159: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t3811 = t1089 * t175 * t3809;
    let t3812 = t384 * t3811;
    let t3814 = t330 * t1080;
    let t3816 = t987 * t363;
    let t3827 = 0.19756347548806534796e1_f64 * t3243 * t453;
    let t3828 = t159 * t1210;
    (t3811, t3812, t3814, t3816, t3827, t3828)
}

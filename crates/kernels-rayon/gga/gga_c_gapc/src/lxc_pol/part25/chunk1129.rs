//! GGA_C_GAPC lxc pol — lxc_pol part 25 (v4rho2sigma2_4) CSE chunk 1129/1444 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part25_v4rho2sigma2_4_chunk1129(t126: f64, t277: f64, t1038: f64, t18105: f64, t2763: f64, t442: f64, t966: f64, t3074: f64, t7592: f64, t7877: f64, t28415: f64, t286: f64) -> (f64, f64, f64, f64) {
    let t29576 = t277 * t126;
    let t29582 = t2763 * t966 * t1038 * t18105 * t442;
    let t29654 = t7592 * t3074 * t7877;
    let t29664 = t28415 * t286;
    (t29576, t29582, t29654, t29664)
}

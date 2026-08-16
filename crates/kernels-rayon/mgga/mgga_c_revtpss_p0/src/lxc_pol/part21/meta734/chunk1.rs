//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2583/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2583(t10009: f64, t1364: f64, t786: f64, t3899: f64, t4078: f64, t689: f64, t10162: f64, t9303: f64, t3903: f64, t9292: f64, t1445: f64, t2439: f64, t9640: f64) -> (f64, f64, f64, f64, f64) {
    let t47490 = t786 * t10009 * t1364;
    let t47493 = t689 * t3899 * t4078;
    let t47495 = t9303 * t10162;
    let t47497 = t9292 * t3903;
    let t47500 = t2439 * t9640 * t1445;
    (t47490, t47493, t47495, t47497, t47500)
}

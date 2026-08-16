//! GGA_C_GAPLOC lxc pol — lxc_pol part 53 (v4rhosigma3_18) CSE chunk 992/1072 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part53_v4rhosigma3_18_chunk992(t3718: f64, t6553: f64, t12148: f64, t2355: f64, t1339: f64, t1537: f64, t46849: f64, t590: f64, t1441: f64, t493: f64, t475: f64) -> (f64, f64, f64, f64, f64) {
    let t47790 = t6553 * t3718;
    let t47791 = t2355 * t12148;
    let t47794 = t1537 * t1339 * t46849 * t590;
    let t47800 = t1441 * t493 * t46849 * t590;
    let t47803 = t46849 * t475;
    (t47790, t47791, t47794, t47800, t47803)
}

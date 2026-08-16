//! GGA_C_GAPLOC lxc pol — lxc_pol part 18 (v4rho2sigma2_1) CSE chunk 1050/1436 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part18_v4rho2sigma2_1_chunk1050(t1422: f64, t161: f64, t1353: f64, t2486: f64, t4624: f64, t1428: f64, t4398: f64, t197: f64, t2293: f64, t1: f64, t20073: f64, t493: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t20901 = t1422 * t161;
    let t20902 = t20901 * t1353;
    let t20954 = t4624 * t2486;
    let t20957 = t4398 * t1428;
    let t21004 = t197 * t2293;
    let t21005 = t21004 * t1;
    let t21042 = t493 * t20073;
    (t20901, t20902, t20954, t20957, t21004, t21005, t21042)
}

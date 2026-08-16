//! GGA_C_GAPC lxc pol — lxc_pol part 31 (v4rho2sigma2_10) CSE chunk 1241/1447 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part31_v4rho2sigma2_10_chunk1241(t1023: f64, t1386: f64, t3669: f64, t11578: f64, t1952: f64, t619: f64, t1030: f64, t11428: f64, t11591: f64, t1461: f64, t505: f64, t11439: f64, t129: f64, t19670: f64) -> (f64, f64, f64, f64) {
    let t34622 = t1386 * t3669 * t1023;
    let t34625 = t11578 * t1952 * t619;
    let t34630 = t1030 * t1461 * t11428 * t505 * t11591;
    let t34633 = t19670 * t129 * t11439;
    (t34622, t34625, t34630, t34633)
}

//! GGA_C_GAPC lxc pol — lxc_pol part 36 (v4rho2sigma2_15) CSE chunk 835/1328 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part36_v4rho2sigma2_15_chunk835(t1092: f64, t9980: f64, t3293: f64, t3297: f64, t869: f64, t134: f64, t2299: f64, t941: f64, t3405: f64, t3403: f64, t2639: f64, t9832: f64) -> (f64, f64, f64, f64, f64) {
    let t9981 = t1092 * t9980;
    let t9986 = t869 * t3293 * t3297;
    let t9988 = t134 * t2299;
    let t9989 = t941 * t9988;
    let t9990 = t3405 * t9989;
    let t9991 = t3403 * t9990;
    let t9993 = t9832 * t2639;
    (t9981, t9986, t9990, t9991, t9993)
}

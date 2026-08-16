//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 906/1242 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk906(t16679: f64, t1896: f64, t587: f64, t590: f64, t1661: f64, t1664: f64, t1620: f64, t5455: f64, t5493: f64, t1879: f64, t5346: f64, t2735: f64, t616: f64, t618: f64) -> (f64, f64, f64, f64, f64) {
    let t17094 = 8.0_f64 / 15.0_f64 * t587 * t590 * t1896 * t16679;
    let t17098 = 4.0_f64 / 9.0_f64 * t587 * t1661 * t1664 * t16679;
    let t17100 = t1620 * t5493 * t5455;
    let t17101 = 64.0_f64 / 15.0_f64 * t17100;
    let t17102 = t1879 * t5346;
    let t17103 = 32.0_f64 / 15.0_f64 * t17102;
    let t17105 = t616 * t2735 * t618;
    (t17094, t17098, t17101, t17103, t17105)
}

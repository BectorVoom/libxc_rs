//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 899/1242 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk899(t1413: f64, t1416: f64, t5002: f64, t11: f64, t1691: f64, t1642: f64, t16986: f64, t4373: f64, t5028: f64, t5063: f64, t5089: f64, t16973: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t17001 = t5002 * t1413 * t1416;
    let t17003 = t11 * t1691 * t17001;
    let t17005 = t1642 * t16986;
    let t17007 = t11 * t1691 * t17005;
    let t17009 = t5028 * t4373;
    let t17011 = t11 * t1691 * t17009;
    let t17014 = t5063 * t1413 * t1416;
    let t17016 = t11 * t5089 * t17014;
    let t17018 = t5002 * t16973;
    (t17001, t17003, t17005, t17007, t17009, t17011, t17014, t17016, t17018)
}

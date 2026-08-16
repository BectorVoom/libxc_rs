//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 922/1242 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk922(t1627: f64, t5490: f64, t5225: f64, t1640: f64, t16973: f64, t5401: f64, t639: f64, t4913: f64, t5506: f64, t16669: f64, t5008: f64, t587: f64, t590: f64) -> (f64, f64, f64, f64, f64) {
    let t17285 = 16.0_f64 / 5.0_f64 * t1627 * t5490;
    let t17287 = 32.0_f64 / 15.0_f64 * t1627 * t5225;
    let t17291 = 16.0_f64 / 3.0_f64 * t639 * t1640 * t5401 * t16973;
    let t17293 = 16.0_f64 / 5.0_f64 * t4913 * t5506;
    let t17297 = 32.0_f64 / 15.0_f64 * t587 * t590 * t5008 * t16669;
    (t17285, t17287, t17291, t17293, t17297)
}

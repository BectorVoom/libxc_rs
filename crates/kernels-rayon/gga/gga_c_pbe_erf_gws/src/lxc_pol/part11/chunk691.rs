//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 691/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk691(t1114: f64, t6644: f64, t1109: f64, t931: f64, t6566: f64, t1136: f64, t6228: f64, t2106: f64, t1140: f64, t6480: f64, t1125: f64, t6616: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t9035 = t1114 * t6644;
    let t9056 = t1109 * t931;
    let t9119 = t1114 * t6566;
    let t9144 = t6228 * t1136;
    let t9150 = t1109 * t2106;
    let t9176 = t6480 * t1140;
    let t9182 = t1125 * t6616;
    (t9035, t9056, t9119, t9144, t9150, t9176, t9182)
}

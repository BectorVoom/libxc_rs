//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 675/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk675(t1076: f64, t1365: f64, t153: f64, t1333: f64, t960: f64, t1438: f64, t2515: f64, t409: f64, t1326: f64, t959: f64, t40: f64, t1444: f64, t2506: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t7981 = t153 * t1365 * t1076;
    let t7986 = t1333 * t960;
    let t7988 = t1438 * t960;
    let t7990 = t409 * t2515;
    let t7996 = t959 * t1326;
    let t7997 = t40 * t7996;
    let t8004 = t2506 * t1444;
    (t7981, t7986, t7988, t7990, t7996, t7997, t8004)
}

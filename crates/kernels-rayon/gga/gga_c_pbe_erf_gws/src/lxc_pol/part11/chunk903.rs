//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 903/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk903(t18442: f64, t470: f64, t4800: f64, t174: f64, t388: f64, t405: f64, t837: f64, t1290: f64, t1294: f64, t331: f64, t1305: f64, t1314: f64, t1319: f64) -> (f64, f64, f64, f64) {
    let t18445 = 0.69263023597503453196e2_f64 * t470 * t4800 * t18442;
    let t18452 = 0.22161481481481481481e0_f64 * t174 * t837 * t388 * t405;
    let t18456 = 0.2291123905095794067e1_f64 * t174 * t331 * t1290 * t1294;
    let t18460 = 0.21053604230838734656e2_f64 * t470 * t1319 * t1314 * t1305;
    (t18445, t18452, t18456, t18460)
}

//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 17 (v4rho3sigma_5) CSE chunk 913/1352 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part17_v4rho3sigma_5_chunk913(t1076: f64, t1365: f64, t153: f64, t2513: f64, t414: f64, t4547: f64, t1333: f64, t960: f64, t1438: f64, t2515: f64, t409: f64, t4602: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t7981 = t153 * t1365 * t1076;
    let t7983 = t414 * t2513;
    let t7984 = 8.0_f64 * t7983;
    let t7985 = 4.0_f64 * t4547;
    let t7986 = t1333 * t960;
    let t7987 = 20.0_f64 * t7986;
    let t7988 = t1438 * t960;
    let t7989 = 32.0_f64 * t7988;
    let t7990 = t409 * t2515;
    let t7991 = 8.0_f64 * t7990;
    let t7992 = 2.0_f64 * t4602;
    (t7981, t7984, t7985, t7987, t7989, t7991, t7992)
}

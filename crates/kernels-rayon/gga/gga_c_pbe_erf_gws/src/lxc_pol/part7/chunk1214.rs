//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1214/1242 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk1214(t6249: f64, t6542: f64, t20135: f64, t20137: f64, t20619: f64, t21065: f64, t21528: f64, t21534: f64, t21537: f64, t21540: f64, t21544: f64, t21563: f64, t2343: f64, t2345: f64, t6220: f64, t6282: f64, t902: f64, t904: f64, t905: f64, t914: f64, t916: f64, t9665: f64) -> (f64, f64) {
    let t21564 = t6542 * t6249;
    let t21565 = 7.0_f64 / 12.0_f64 * t21564;
    let t21566 = t21528 - t21534 - 7.0_f64 / 96.0_f64 * t21537 - t21540 + t21544 - t914 * t916 * t904 * t20619 / 512.0_f64 + t2343 * t9665 * t21065 / 32.0_f64 + t902 * t905 * t20135 * t20137 / 192.0_f64 + t2343 * t2345 * t6282 * t6220 / 64.0_f64 - t21563 - t21565;
    (t21565, t21566)
}

//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 922/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk922(t1327: f64, t1336: f64, t408: f64, t4259: f64, t88: f64, t414: f64, t4743: f64, t1332: f64, t274: f64, t169: f64, t18411: f64, t289: f64) -> (f64, f64, f64, f64, f64) {
    let t18969 = t1336 * t1327;
    let t18970 = 72.0_f64 * t18969;
    let t18972 = t408 * t4259 * t88;
    let t18973 = 1920.0_f64 * t18972;
    let t18977 = 16.0_f64 * t414 * t4743;
    let t18995 = 0.6399008129061525636e1_f64 * t1332 * t274;
    let t18998 = 0.31835665774679373271e-1_f64 * t169 * t289 * t18411;
    (t18970, t18973, t18977, t18995, t18998)
}

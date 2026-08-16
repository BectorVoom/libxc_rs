//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1175/1332 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1175(t1140: f64, t6320: f64, t1137: f64, t5907: f64, t145: f64, t5506: f64, t1150: f64, t1318: f64, t16425: f64, t16427: f64, t16438: f64, t16440: f64, t16442: f64, t16444: f64, t16446: f64, t18129: f64, t301: f64, t335: f64, t6387: f64, t839: f64, t960: f64) -> f64 {
    let t21303 = t1140 * t6320;
    let t21305 = t1137 * t5907;
    let t21307 = t145 * t5506;
    let t21319 = 0.80031500487063509016e-2_f64 * t16425 - 0.34299214494455789578e-2_f64 * t16427 + 0.25724410870841842183e-2_f64 * t16438 + 0.68598428988911579156e-2_f64 * t16440 - 0.68598428988911579156e-2_f64 * t16442 + 0.68598428988911579156e-2_f64 * t16444 + 0.68598428988911579156e-2_f64 * t16446 - 7.0_f64 / 36.0_f64 * t21303 - 7.0_f64 / 72.0_f64 * t21305 + t1150 * t960 * t21307 * t301 / 8.0_f64 + t1150 * t960 * t6387 * t839 / 16.0_f64 + t335 * t18129 * t1318 / 12.0_f64;
    t21319
}

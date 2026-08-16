//! GGA_C_ACGGAP lxc pol — lxc_pol part 13 (v4rho3sigma_5) CSE chunk 476/1213 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part13_v4rho3sigma_5_chunk476(t2282: f64, t578: f64, t2041: f64, t500: f64, t1969: f64, t1971: f64, t1987: f64, t1990: f64, t1996: f64, t2000: f64, t2011: f64, t2014: f64, t2018: f64, t2258: f64, t2261: f64, t2265: f64, t2269: f64, t2271: f64, t2275: f64, t2279: f64) -> f64 {
    let t2283 = t578 * t2282;
    let t2285 = t2041 * t500;
    let t2287 = t1969 - t1971 + t1987 - t1990 - t1996 - t2000 - 0.17149607247227894789e-2_f64 * t2258 - t2011 + t2014 + t2261 / 96.0_f64 - 0.10718504529517434243e-3_f64 * t2265 + 0.15724046144802076034e-3_f64 * t2269 + t2018 - t2271 / 96.0_f64 - t2275 / 128.0_f64 - t2279 / 384.0_f64 - 0.38203125e-2_f64 * t2283 - t2285 / 48.0_f64;
    t2287
}

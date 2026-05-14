//! GGA_C_ACGGAP lxc pol — lxc_pol part 13 (v4rho3sigma_5) CSE chunk 468/1066 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part13_v4rho3sigma_5_chunk468<F: Float>(t2068: F, t2268: F, t515: F, t570: F, t495: F, t579: F, t336: F, t2046: F, t513: F, t599: F, t578: F, t137: F, t535: F, t2041: F, t500: F, t1969: F, t1971: F, t1987: F, t1990: F, t1996: F, t2000: F, t2011: F, t2014: F, t2018: F, t2258: F, t2261: F, t2265: F) -> (F, F, F, F) {
    let t2269 = t2068 * t2268;
    let t2271 = t570 * t515;
    let t2273 = t579 * t495;
    let t2274 = t336 * t2273;
    let t2275 = t2046 * t2274;
    let t2277 = t599 * t513;
    let t2278 = t336 * t2277;
    let t2279 = t578 * t2278;
    let t2282 = t336 * t535 * t137;
    let t2283 = t578 * t2282;
    let t2285 = t2041 * t500;
    let t2287 = t1969 - t1971 + t1987 - t1990 - t1996 - t2000 - 0.17149607247227894789e-2 * t2258 - t2011 + t2014 + t2261 / 96.0 - 0.10718504529517434243e-3 * t2265 + 0.15724046144802076034e-3 * t2269 + t2018 - t2271 / 96.0 - t2275 / 128.0 - t2279 / 384.0 - 0.38203125e-2 * t2283 - t2285 / 48.0;
    (t2274, t2278, t2282, t2287)
}

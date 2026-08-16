//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 360/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk360(t2271: f64, t500: f64, t2231: f64, t499: f64, t498: f64, t1504: f64, t2152: f64, t381: f64, t493: f64, t2260: f64, t2264: f64, t2268: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t2272 = t2271 * t500;
    let t2274 = t499 * t2231;
    let t2275 = t498 * t2274;
    let t2276 = t1504 * t2275;
    let t2278 = t381 * t2152;
    let t2279 = t498 * t2278;
    let t2280 = t493 * t2279;
    let t2282 = t2260 / 16.0_f64 - t2264 / 16.0_f64 + t2268 / 24.0_f64 - t2272 / 256.0_f64 + t2276 / 256.0_f64 - t2280 / 192.0_f64;
    (t2272, t2275, t2276, t2279, t2280, t2282)
}

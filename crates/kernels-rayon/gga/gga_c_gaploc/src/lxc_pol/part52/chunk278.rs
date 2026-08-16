//! GGA_C_GAPLOC lxc pol — lxc_pol part 52 (v4rhosigma3_17) CSE chunk 278/1013 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part52_v4rhosigma3_17_chunk278(t535: f64, t876: f64, t130: f64, t455: f64, t145: f64, t459: f64, t1234: f64, t1232: f64, t1242: f64, t1247: f64, t1240: f64, t467: f64, t864: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let pi = (M_PI as f64);
    let t2269 = t535 * t876;
    let t2272 = t130 * t455;
    let t2274 = t2272 * t145 * t459;
    let t2276 = 1.0_f64 / t1234;
    let t2277 = t1232 * t2276;
    let t2278 = t2277 * t1242;
    let t2280 = t1247 * t1232;
    let t2281 = t2276 * t1240;
    let t2282 = t2281 * pi;
    let t2283 = t2280 * t2282;
    let t2285 = t864 * t467;
    (t2269, t2272, t2274, t2276, t2277, t2278, t2281, t2282, t2283, t2285)
}

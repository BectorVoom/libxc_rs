//! GGA_C_GAPLOC lxc pol — lxc_pol part 42 (v4rhosigma3_7) CSE chunk 278/1012 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part42_v4rhosigma3_7_chunk278<F: Float>(t535: F, t876: F, t130: F, t455: F, t145: F, t459: F, t1234: F, t1232: F, t1242: F, t1247: F, t1240: F, t467: F, t864: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t2269 = t535 * t876;
    let t2272 = t130 * t455;
    let t2274 = t2272 * t145 * t459;
    let t2276 = F::new(1.0) / t1234;
    let t2277 = t1232 * t2276;
    let t2278 = t2277 * t1242;
    let t2280 = t1247 * t1232;
    let t2281 = t2276 * t1240;
    let t2282 = t2281 * M_PI;
    let t2283 = t2280 * t2282;
    let t2285 = t864 * t467;
    (t2269, t2272, t2274, t2276, t2277, t2278, t2281, t2282, t2283, t2285)
}

//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 460/1272 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk460<F: Float>(t2258: F, t2259: F, t2266: F, t2272: F, t2276: F, t2281: F, t2282: F, t2286: F, t2289: F, t2292: F, t970: F, t973: F, t349: F, t972: F, t346: F, t979: F) -> (F, F, F, F, F) {
    let t2294 = t2258 + 0.12925555555555555555e1 * t2259 - 0.12925555555555555555e1 * t2266 + 0.38776666666666666666e1 * t2272 - 0.19388333333333333333e1 * t2276 + t2281 + 0.1642e-2 * t2282 - 0.4105e-3 * t2286 + 0.2463e-2 * t2289 - 0.12315e-2 * t2292;
    let t2296 = t970 * t973;
    let t2300 = 1.0 / t972 / t349;
    let t2301 = t346 * t2300;
    let t2302 = t979 * t979;
    (t2294, t2296, t2300, t2301, t2302)
}

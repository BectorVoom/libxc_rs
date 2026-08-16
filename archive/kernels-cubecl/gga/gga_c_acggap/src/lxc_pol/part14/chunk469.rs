//! GGA_C_ACGGAP lxc pol — lxc_pol part 14 (v4rho3sigma_6) CSE chunk 469/1223 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part14_v4rho3sigma_6_chunk469<F: Float>(t2068: F, t2268: F, t515: F, t570: F, t495: F, t579: F, t336: F, t2046: F, t513: F, t599: F, t578: F, t137: F, t535: F) -> (F, F, F, F, F, F, F) {
    let t2269 = t2068 * t2268;
    let t2271 = t570 * t515;
    let t2273 = t579 * t495;
    let t2274 = t336 * t2273;
    let t2275 = t2046 * t2274;
    let t2277 = t599 * t513;
    let t2278 = t336 * t2277;
    let t2279 = t578 * t2278;
    let t2282 = t336 * t535 * t137;
    (t2269, t2271, t2274, t2275, t2278, t2279, t2282)
}

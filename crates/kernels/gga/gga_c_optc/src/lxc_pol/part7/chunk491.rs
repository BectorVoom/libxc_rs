//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 491/1272 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk491<F: Float>(t2474: F, t2476: F, t845: F, t2257: F, t2259: F, t2266: F, t2272: F, t2276: F, t805: F, t809: F, t248: F, t808: F, t243: F, t817: F) -> (F, F, F, F, F, F, F, F) {
    let t2477 = t2474 * t2476;
    let t2479 = 0.17315755899375863299e2 * t845 * t2477;
    let t2480 = 0.22831111111111111111e-1 * t2257;
    let t2485 = t2480 + 0.11415555555555555555e-1 * t2259 - 0.11415555555555555555e-1 * t2266 + 0.34246666666666666666e-1 * t2272 - 0.17123333333333333333e-1 * t2276;
    let t2488 = t805 * t809;
    let t2491 = t808 * t248;
    let t2492 = 1.0 / t2491;
    let t2493 = t243 * t2492;
    let t2494 = t817 * t817;
    (t2477, t2479, t2485, t2488, t2491, t2492, t2493, t2494)
}

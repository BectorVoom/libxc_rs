//! GGA_C_ACGGAP lxc pol — lxc_pol part 13 (v4rho3sigma_5) CSE chunk 517/1066 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part13_v4rho3sigma_5_chunk517<F: Float>(t1008: F, t1200: F, t1195: F, t997: F, t336: F, t360: F, t1017: F, t322: F, t1459: F, t398: F, t384: F, t1016: F, t141: F) -> (F, F, F, F, F, F, F, F) {
    let t3271 = t1008 * t1200;
    let t3273 = t1008 * t1195;
    let t3280 = t997 * t1200;
    let t3282 = t336 * t360;
    let t3290 = t1017 * t322;
    let t3292 = t398 * t1459 * t3290;
    let t3293 = t384 * t3292;
    let t3300 = t141 * t1016;
    (t3271, t3273, t3280, t3282, t3290, t3292, t3293, t3300)
}

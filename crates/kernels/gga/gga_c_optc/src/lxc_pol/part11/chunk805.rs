//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 805/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk805<F: Float>(t16393: F, t7129: F, t16397: F, t2126: F, t10002: F, t10008: F, t13482: F, t13487: F, t16394: F, t16398: F, t2124: F, t2168: F, t7091: F, t7094: F, t9913: F, t9915: F) -> (F, F, F) {
    let t16554 = t7129 * t16393;
    let t16557 = t2126 * t16397;
    let t16566 = 0.18137053605011111023e0 * t2168 * t16398 - 0.90685268025055555117e0 * t2168 * t16394 - 0.15647690681619764138e1 * t2124 * t16554 + 0.52158968938732547127e0 * t2124 * t16557 - t7091 - t7094 - 0.23981215322181357908e1 * t9913 - 0.40568086952347536654e1 * t9915 - 0.24340852171408521993e1 * t13482 - 0.16927916698010370288e1 * t13487 - 0.11990607661090678954e1 * t10002 - 0.20284043476173768327e1 * t10008;
    (t16554, t16557, t16566)
}

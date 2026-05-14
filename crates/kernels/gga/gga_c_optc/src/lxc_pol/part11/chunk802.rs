//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 802/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk802<F: Float>(t16477: F, t2024: F, t675: F, t127: F, t16411: F, t3519: F, t16419: F, t3491: F, t16376: F, t696: F, t16416: F, t16438: F, t16464: F, t16471: F, t16474: F, t16479: F, t2113: F, t2159: F, t673: F, t686: F, t695: F, t6993: F, t7002: F, t705: F) -> (F, F, F, F, F, F) {
    let t16482 = t16477 * t2024;
    let t16483 = t675 * t16482;
    let t16486 = t16477 * t127;
    let t16487 = t675 * t16486;
    let t16490 = t3519 * t16411;
    let t16493 = t3491 * t16419;
    let t16496 = t696 * t16376;
    let t16499 = -0.20863587575493018851e1 * t686 * t16464 - 0.30228422675018518372e-1 * t705 * t16438 - 0.90685268025055555116e0 * t705 * t16416 + 0.13602790203758333267e0 * t2159 * t16471 - 0.18137053605011111023e0 * t6993 * t16474 - 0.52158968938732547127e0 * t7002 * t16479 + 0.52158968938732547127e0 * t2113 * t16483 - 0.86931614897887578546e-1 * t673 * t16487 + 0.45342634012527777558e0 * t705 * t16490 + 0.15647690681619764138e1 * t686 * t16493 - 0.15114211337509259186e-1 * t695 * t16496;
    (t16483, t16487, t16490, t16493, t16496, t16499)
}

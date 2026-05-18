//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 852/1451 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk852<F: Float>(t16323: F, t5: F, t6879: F, t675: F, t2024: F, t127: F, t16411: F, t3519: F, t16419: F, t3491: F, t16376: F, t696: F) -> (F, F, F, F, F, F) {
    let t16477 = t5 * t16323;
    let t16478 = t16477 * t6879;
    let t16479 = t675 * t16478;
    let t16482 = t16477 * t2024;
    let t16483 = t675 * t16482;
    let t16486 = t16477 * t127;
    let t16487 = t675 * t16486;
    let t16490 = t3519 * t16411;
    let t16493 = t3491 * t16419;
    let t16496 = t696 * t16376;
    (t16479, t16483, t16487, t16490, t16493, t16496)
}

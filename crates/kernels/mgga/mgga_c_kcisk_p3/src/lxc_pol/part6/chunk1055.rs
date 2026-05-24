//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 1055/1086 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk1055<F: Float>(t14545: F, t31272: F, t21315: F, t8241: F, t30205: F, t381: F, t498: F, t493: F, t21066: F, t8268: F, t30962: F, t6317: F) -> (F, F, F, F, F) {
    let t31273 = t14545 * t31272;
    let t31275 = t21315 * t8241;
    let t31277 = t381 * t30205;
    let t31278 = t498 * t31277;
    let t31279 = t493 * t31278;
    let t31281 = t21066 * t8268;
    let t31283 = t6317 * t30962;
    (t31273, t31275, t31279, t31281, t31283)
}

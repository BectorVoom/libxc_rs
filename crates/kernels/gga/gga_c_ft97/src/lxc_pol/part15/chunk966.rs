//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 966/1222 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk966<F: Float>(t80002: F, t807: F, t21233: F, t213: F, t21359: F, t458: F, t21366: F, t21356: F, t21363: F, t21352: F, t21370: F, t21249: F, t237: F, t677: F) -> (F, F, F, F, F, F, F, F, F) {
    let t80003 = t807 * t80002;
    let t80012 = t213 * t21233;
    let t80029 = t458 * t21359;
    let t80031 = t458 * t21366;
    let t80087 = t458 * t21356;
    let t80089 = t458 * t21363;
    let t80091 = t458 * t21352;
    let t80096 = t458 * t21370;
    let t80127 = t677 * t237 * t21249;
    (t80003, t80012, t80029, t80031, t80087, t80089, t80091, t80096, t80127)
}

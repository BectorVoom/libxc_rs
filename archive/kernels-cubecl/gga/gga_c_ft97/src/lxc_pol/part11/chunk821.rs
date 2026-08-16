//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 821/1173 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk821<F: Float>(t2492: F, t265: F, t9802: F, t332: F, t505: F, t2440: F, t327: F, t10845: F, t2347: F, t2360: F, t2923: F, t287: F, t4061: F) -> (F, F, F, F, F, F, F) {
    let t14196 = t2492 * t265;
    let t14200 = t9802 * t265;
    let t14408 = t332 * t505;
    let t14487 = t2440 * t327;
    let t14514 = t10845 * t2347;
    let t14519 = t2923 * t2360;
    let t14763 = t4061 * t287;
    (t14196, t14200, t14408, t14487, t14514, t14519, t14763)
}

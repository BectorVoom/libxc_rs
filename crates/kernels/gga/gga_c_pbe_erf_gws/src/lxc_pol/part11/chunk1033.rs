//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1033/1141 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk1033<F: Float>(t26204: F, t47355: F, t47359: F, t47363: F, t47364: F, t47366: F, t47368: F, t47369: F, t47376: F, t47381: F, t47385: F, t16553: F, t16556: F, t16561: F, t47387: F, t47389: F, t47506: F, t47507: F, t47511: F, t47515: F, t47519: F, t47523: F) -> (F, F) {
    let t48625 = -t47355 - t47359 - t47363 - t47364 + t47366 + 0.86568330898918747016e0 * t26204 - t47368 - t47369 + t47376 + t47381 - t47385;
    let t48626 = -t47387 - t47389 - t47506 - t47507 - t47511 - t47515 + t47519 - t47523 + t16553 + t16556 - t16561;
    (t48625, t48626)
}

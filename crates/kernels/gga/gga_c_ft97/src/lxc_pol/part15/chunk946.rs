//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 946/1222 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk946<F: Float>(t21052: F, t8675: F, t21056: F, t21075: F, t21068: F, t21064: F, t21059: F, t21062: F, t2253: F, t21031: F, t21040: F, t21044: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t76056 = t8675 * t21052;
    let t76062 = t8675 * t21056;
    let t76101 = t8675 * t21075;
    let t76126 = t8675 * t21068;
    let t76128 = t8675 * t21064;
    let t76130 = t8675 * t21059;
    let t76199 = t2253 * t21062;
    let t76210 = t2253 * t21031;
    let t76221 = t2253 * t21040;
    let t76232 = t2253 * t21044;
    (t76056, t76062, t76101, t76126, t76128, t76130, t76199, t76210, t76221, t76232)
}

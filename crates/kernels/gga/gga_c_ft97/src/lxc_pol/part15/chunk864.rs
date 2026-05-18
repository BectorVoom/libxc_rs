//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 864/1222 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk864<F: Float>(t100: F, t38482: F, t104: F, t38061: F, t89: F, t487: F, t7800: F, t179: F, t37406: F, t70: F, t8119: F, t37355: F) -> (F, F, F, F, F, F) {
    let t39272 = t38482 * t100;
    let t39317 = F::new(280.0) / F::new(243.0) * t89 * t38061 * t104;
    let t39345 = t487 * t7800;
    let t39417 = t179 * t37406;
    let t39430 = t70 * t8119;
    let t39431 = t179 * t37355;
    (t39272, t39317, t39345, t39417, t39430, t39431)
}

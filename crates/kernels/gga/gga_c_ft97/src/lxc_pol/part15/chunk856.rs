//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 856/1222 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk856<F: Float>(t37406: F, t82: F, t23: F, t32075: F, t1609: F, t1610: F, t1613: F, t8119: F, t1557: F, t37355: F, t422: F, t7800: F) -> (F, F, F, F, F, F) {
    let t37407 = t82 * t37406;
    let t37429 = t23 * t32075;
    let t37481 = t1613 * t1610 * t1609;
    let t37730 = t8119 * t37406;
    let t37748 = F::cast_from(1.0_f64) / t23 / t1557;
    let t37749 = t37748 * t37355;
    let t37765 = t422 * t7800;
    (t37407, t37429, t37481, t37730, t37749, t37765)
}

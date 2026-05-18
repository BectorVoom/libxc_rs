//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 870/1222 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk870<F: Float>(t143: F, t37406: F, t37352: F, t2: F, t32905: F, t355: F, t7368: F, t525: F, t7760: F, t1554: F, t1984: F, t11176: F, t151: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t40294 = t143 * t37406;
    let t40337 = t37352 * t143;
    let t40379 = t32905 * t2;
    let t40424 = t355 * t7368;
    let t40425 = t40424 * t2;
    let t40436 = t7760 * t525;
    let t40437 = t40436 * t2;
    let t40465 = t1554 * t1984;
    let t40466 = t40465 * t2;
    let t40485 = F::new(280.0) / F::new(81.0) * t11176 * t151;
    (t40294, t40337, t40379, t40424, t40425, t40436, t40437, t40465, t40466, t40485)
}

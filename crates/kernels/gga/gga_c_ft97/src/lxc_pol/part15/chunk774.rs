//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 774/1067 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk774<F: Float>(t1586: F, t22: F, t36452: F, t37991: F, t96: F, t1554: F, t2: F, t355: F, t7241: F, t369: F, t7760: F, t32075: F, t11176: F, t94: F, t37406: F, t37352: F, t82: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t38456 = 1.0 / t96 / t37991 / t22 / t1586 / t36452 / 96.0;
    let t38463 = t1554 * t1586;
    let t38464 = t38463 * t2;
    let t38477 = t355 * t7241;
    let t38478 = t38477 * t2;
    let t38482 = t7760 * t369;
    let t38483 = t38482 * t2;
    let t38508 = t32075 * t2;
    let t38525 = 280.0 / 81.0 * t11176 * t94;
    let t38549 = t2 * t37406;
    let t38570 = t37352 * t82;
    (t38456, t38463, t38464, t38477, t38478, t38482, t38483, t38508, t38525, t38549, t38570)
}

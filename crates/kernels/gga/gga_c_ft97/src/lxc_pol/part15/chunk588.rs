//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 588/1222 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk588<F: Float>(t209: F, t2247: F, t228: F, t231: F, t191: F, t2360: F, t9570: F, t2440: F, t9577: F, t2347: F, t703: F, t693: F) -> (F, F, F, F, F, F, F) {
    let t9634 = t209 * t2247;
    let t9636 = t228 * t9634 * t231;
    let t9637 = F::new(0.70937342644032921812e-2) * t9636;
    let t9651 = F::new(1.0) / t191 / t2360;
    let t9652 = t9651 * t9570;
    let t9657 = t2440 * t9577;
    let t9665 = t703 * t2347;
    let t9680 = t693 * t693;
    (t9636, t9637, t9651, t9652, t9657, t9665, t9680)
}

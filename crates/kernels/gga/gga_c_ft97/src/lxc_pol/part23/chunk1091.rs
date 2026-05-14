//! GGA_C_FT97 lxc pol — lxc_pol part 23 (v4rho3sigma_8) CSE chunk 1091/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part23_v4rho3sigma_8_chunk1091<F: Float>(t6056: F, t96535: F, t6055: F, t6054: F, t697: F, t13519: F, t6028: F, t2440: F, t420: F, t679: F, t1609: F, t209: F, t9: F, t2387: F, t2378: F, t703: F) -> (F, F, F, F, F, F, F, F) {
    let t96536 = t96535 * t6056;
    let t96537 = t6055 * t96536;
    let t96539 = t6054 * t697;
    let t96576 = t13519 * t6028;
    let t96593 = t420 * t2440 * t679;
    let t96598 = t1609 * t209;
    let t96599 = t9 * t96598;
    let t96600 = t2387 * t96599;
    let t96602 = t420 * t703 * t2378;
    (t96536, t96537, t96539, t96576, t96593, t96599, t96600, t96602)
}

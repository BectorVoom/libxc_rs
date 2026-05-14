//! GGA_C_FT97 lxc pol — lxc_pol part 18 (v4rho3sigma_3) CSE chunk 1288/1396 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part18_v4rho3sigma_3_chunk1288<F: Float>(t51: F, t5827: F, t23831: F, t1290: F, t16762: F, t538: F, t3379: F, t422: F, t22515: F, t23826: F, t34871: F, t12374: F, t23714: F, t5829: F, t6608: F, t92557: F) -> (F, F, F, F, F, F, F) {
    let t104689 = t5827 * t51;
    let t104690 = t23831 * t104689;
    let t104692 = t1290 * t16762 * t538;
    let t104695 = t422 * t3379;
    let t104701 = t22515 * t34871 * t23826;
    let t104704 = t12374 * t23714;
    let t104712 = t5829 * t92557 * t6608;
    (t104689, t104690, t104692, t104695, t104701, t104704, t104712)
}

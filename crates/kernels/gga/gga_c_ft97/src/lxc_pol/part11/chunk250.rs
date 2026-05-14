//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 250/1030 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk250<F: Float>(t265: F, t713: F, t729: F, t251: F, t249: F, t458: F, t241: F, t665: F) -> (F, F, F, F) {
    let t731 = t729 * t265 * t713;
    let t734 = 1.0 / t251;
    let t736 = t458 * t249 / 3.0;
    let t737 = t665 * t241;
    (t731, t734, t736, t737)
}

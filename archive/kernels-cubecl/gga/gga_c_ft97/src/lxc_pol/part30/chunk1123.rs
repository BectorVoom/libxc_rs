//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 1123/1184 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk1123<F: Float>(t1200: F, t153074: F, t285: F, t153047: F, t1208: F, t7464: F, t142832: F, t811: F, t820: F, t4125: F, t52: F, t7457: F) -> (F, F, F, F, F, F) {
    let t153130 = t1200 * t153074;
    let t153133 = t285 * t153074;
    let t153136 = t285 * t153047;
    let t153141 = t7464 * t1208;
    let t153143 = t142832 * t153141 * t811;
    let t153147 = t142832 * t153141 * t820;
    let t153155 = t52 * t7457 * t4125;
    (t153130, t153133, t153136, t153143, t153147, t153155)
}

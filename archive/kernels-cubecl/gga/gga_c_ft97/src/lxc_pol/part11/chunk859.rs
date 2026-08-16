//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 859/1173 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk859<F: Float>(t409: F, t7918: F, t53: F, t7934: F, t1751: F, t3020: F, t408: F, t1655: F) -> (F, F, F, F) {
    let t37611 = t7918 * t409;
    let t37614 = t53 * t7934;
    let t37622 = t3020 * t408 * t1751;
    let t37627 = t1655 * t1655;
    (t37611, t37614, t37622, t37627)
}

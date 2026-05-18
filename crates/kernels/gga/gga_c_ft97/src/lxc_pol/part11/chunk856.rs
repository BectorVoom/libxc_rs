//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 856/1173 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk856<F: Float>(t1609: F, t408: F, t1593: F, t1608: F, t7839: F, t8035: F, t8031: F, t7837: F, t8014: F, t1619: F, t1681: F, t401: F) -> (F, F, F, F, F) {
    let t37523 = t408 * t1609;
    let t37525 = t1608 * t37523 * t1593;
    let t37526 = t8035 * t7839;
    let t37529 = t8031 * t7839;
    let t37537 = t7837 * t8014;
    let t37541 = t1619 * t1681 * t401;
    (t37525, t37526, t37529, t37537, t37541)
}

//! GGA_C_FT97 lxc pol — lxc_pol part 18 (v4rho3sigma_3) CSE chunk 820/1396 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part18_v4rho3sigma_3_chunk820<F: Float>(t428: F, t53: F, t72: F, t5591: F, t1293: F, t409: F, t1602: F) -> (F, F, F, F, F) {
    let t22820 = t53 * t428;
    let t22821 = t72 * t22820;
    let t22822 = t5591 * t22821;
    let t22825 = t409 * t1293;
    let t22826 = t1602 * t22825;
    (t22820, t22821, t22822, t22825, t22826)
}

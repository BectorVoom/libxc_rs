//! GGA_C_FT97 lxc pol — lxc_pol part 18 (v4rho3sigma_3) CSE chunk 796/1396 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part18_v4rho3sigma_3_chunk796<F: Float>(t379: F, t384: F, t22585: F, t1299: F, t388: F, t1701: F, sigma0: F) -> (F, F, F, F) {
    let t22586 = t384 * t379;
    let t22587 = t22585 * t22586;
    let t22590 = t388 * t1299;
    let t22591 = t1701 * sigma0;
    (t22586, t22587, t22590, t22591)
}

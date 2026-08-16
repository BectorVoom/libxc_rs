//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 988/1222 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk988<F: Float>(t10696: F, t22249: F, t22377: F, t8392: F, t22393: F, t22388: F, t1882: F, t22201: F, t22416: F, t22383: F, t22161: F, t312: F) -> (F, F, F, F, F, F, F, F) {
    let t84171 = t10696 * t22249;
    let t84222 = t8392 * t22377;
    let t84283 = t8392 * t22393;
    let t84312 = t8392 * t22388;
    let t84317 = t1882 * t22201;
    let t84331 = t1882 * t22416;
    let t84357 = t8392 * t22383;
    let t84382 = t312 * t22161;
    (t84171, t84222, t84283, t84312, t84317, t84331, t84357, t84382)
}

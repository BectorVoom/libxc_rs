//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 868/1067 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk868<F: Float>(t21978: F, t312: F, t1882: F, t22449: F, t22230: F, t22407: F, t22361: F, t22465: F, t22214: F, t22196: F, t22457: F, t8392: F, t10696: F, t22249: F, t22377: F, t22393: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t83825 = t312 * t21978;
    let t83982 = t1882 * t22449;
    let t83988 = t1882 * t22230;
    let t83990 = t1882 * t22407;
    let t84080 = t1882 * t22361;
    let t84087 = t1882 * t22465;
    let t84138 = t1882 * t22214;
    let t84167 = t1882 * t22196;
    let t84169 = t8392 * t22457;
    let t84171 = t10696 * t22249;
    let t84222 = t8392 * t22377;
    let t84283 = t8392 * t22393;
    (t83825, t83982, t83988, t83990, t84080, t84087, t84138, t84167, t84169, t84171, t84222, t84283)
}

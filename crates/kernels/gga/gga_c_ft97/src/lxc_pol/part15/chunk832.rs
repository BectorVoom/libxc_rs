//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 832/1067 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk832<F: Float>(t173: F, t20069: F, t419: F, t20083: F, t20334: F, t458: F, t20341: F, t1775: F, t20345: F, t20363: F, t1882: F, t20153: F, t20138: F, t20124: F, t37401: F, t89: F) -> (F, F, F, F, F, F, F, F, F) {
    let t74153 = t419 * t173 * t20069;
    let t74162 = t419 * t173 * t20083;
    let t74266 = t458 * t20334;
    let t74268 = t458 * t20341;
    let t74285 = t1775 * t20345;
    let t74287 = t1775 * t20363;
    let t74307 = t1882 * t20153;
    let t74374 = t1882 * t20138;
    let t74377 = t89 * t37401 * t20124;
    (t74153, t74162, t74266, t74268, t74285, t74287, t74307, t74374, t74377)
}

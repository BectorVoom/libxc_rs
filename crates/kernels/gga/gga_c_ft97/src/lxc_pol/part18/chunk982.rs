//! GGA_C_FT97 lxc pol — lxc_pol part 18 (v4rho3sigma_3) CSE chunk 982/1396 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part18_v4rho3sigma_3_chunk982<F: Float>(t26056: F, t83: F, t1339: F, t3103: F, t452: F, t499: F, t6454: F, t110: F, t25846: F, t1882: F, t6475: F, t6544: F, t6484: F, t11552: F, t25929: F, t6480: F) -> (F, F, F, F, F, F, F, F, F) {
    let t26276 = t83 * t26056;
    let t26280 = t452 * t1339 * t3103;
    let t26284 = t452 * t499 * t6454;
    let t26288 = t452 * t110 * t25846;
    let t26291 = t1882 * t6475;
    let t26293 = t1882 * t6544;
    let t26295 = t1882 * t6484;
    let t26297 = t11552 * t25929;
    let t26301 = t1882 * t6480;
    (t26276, t26280, t26284, t26288, t26291, t26293, t26295, t26297, t26301)
}

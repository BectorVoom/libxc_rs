//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 642/1042 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk642<F: Float>(t7124: F, t824: F, t840: F, t871: F, t28855: F, t296: F, t28931: F, t24890: F, t4256: F, t312: F, t7021: F, t684: F, t2874: F, t28842: F, t295: F, t28852: F) -> (F, F, F, F, F, F, F, F, F) {
    let t29245 = t7124 * t824;
    let t29247 = t840 * t871 * t29245;
    let t29250 = t296 * t28855;
    let t29253 = t296 * t28931;
    let t29256 = t24890 * t4256;
    let t29259 = t312 * t7021;
    let t29260 = t29259 * t684;
    let t29261 = t2874 * t29260;
    let t29265 = t295 * t28842 * t312;
    let t29270 = t296 * t28852;
    (t29245, t29247, t29250, t29253, t29256, t29260, t29261, t29265, t29270)
}

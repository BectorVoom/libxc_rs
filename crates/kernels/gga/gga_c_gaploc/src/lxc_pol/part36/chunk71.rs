//! GGA_C_GAPLOC lxc pol — lxc_pol part 36 (v4rhosigma3_1) CSE chunk 71/884 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part36_v4rhosigma3_1_chunk71<F: Float>(t169: F, t279: F, t296: F, t299: F, t247: F, t249: F, t270: F, t33: F, t178: F, t260: F, t110: F, t271: F, t107: F, t183: F, t266: F, t278: F) -> (F, F, F, F, F, F) {
    let t301 = t279 * t296 * t169 * t299;
    let t304 = -t33 + t247 + t249 + 0.76905262301422242837e-2 * t270 * t301;
    let t305 = t260 * t178;
    let t306 = t110 * t271;
    let t312 = 0.58998125e-2 * t305 * t306 - 0.21511666666666666667e-1 * t107 * t183 * t266;
    let t313 = t312 * t278;
    (t301, t304, t305, t306, t312, t313)
}

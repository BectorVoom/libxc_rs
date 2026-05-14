//! MGGA_C_KCISK lxc pol — lxc_pol part 25 (v4rho3sigma_5) CSE chunk 1238/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part25_v4rho3sigma_5_chunk1238<F: Float>(t111270: F, t31898: F, t31901: F, t111229: F, t111234: F, t111238: F, t111243: F, t111249: F, t111253: F, t111255: F, t111260: F, t111264: F, t111268: F, t31893: F, t111252: F, t31902: F) -> (F, F, F) {
    let t111272 = t31901 * t111270 * t31898;
    let t111274 = -0.31250000000000000001e-1 * t111229 + 0.36187500000000000001e-1 * t111234 + 0.69841875000000000003e-2 * t111238 - 0.31250000000000000001e-1 * t111243 - 0.62500000000000000002e-1 * t111249 + 0.24125000000000000001e-1 * t111253 + 0.62500000000000000002e-1 * t111255 - 0.361875e-1 * t111260 - 0.62500000000000000002e-1 * t111264 - 0.24125000000000000001e-1 * t111268 + 0.14583333333333333334e0 * t111272;
    let t111276 = t31893 * t111270 * t31898;
    let t111278 = t31902 * t111252;
    (t111274, t111276, t111278)
}

//! GGA_C_GAPC lxc pol — lxc_pol part 37 (v4rho2sigma2_16) CSE chunk 100/1445 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part37_v4rho2sigma2_16_chunk100<F: Float>(t22: F, t268: F, t159: F, t260: F, t106: F, t269: F, t103: F, t164: F, t266: F, t276: F) -> (F, F, F, F, F) {
    let t299 = t22 * t268;
    let t303 = t260 * t159;
    let t304 = t106 * t269;
    let t310 = F::new(0.58998125e-2) * t303 * t304 - F::new(0.21511666666666666667e-1) * t103 * t164 * t266;
    let t311 = t310 * t276;
    (t299, t303, t304, t310, t311)
}

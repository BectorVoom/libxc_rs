//! GGA_C_GAPC lxc pol — lxc_pol part 36 (v4rho2sigma2_15) CSE chunk 103/1133 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part36_v4rho2sigma2_15_chunk103<F: Float>(t129: F, t315: F, t247: F, t249: F, t284: F, t33: F, t282: F, t311: F) -> (F, F, F) {
    let t316 = t129 * t315;
    let t319 = -t33 + t247 + t249 + 0.28183154870449698953e-3 * t284 * t316;
    let t320 = t311 * t282;
    (t316, t319, t320)
}

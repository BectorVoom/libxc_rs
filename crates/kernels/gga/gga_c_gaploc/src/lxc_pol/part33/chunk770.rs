//! GGA_C_GAPLOC lxc pol — lxc_pol part 33 (v4rho2sigma2_16) CSE chunk 770/1464 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part33_v4rho2sigma2_16_chunk770<F: Float>(t7289: F, t7292: F, t123: F, t2101: F, t5263: F, t883: F, t943: F, t161: F, t2610: F, t2095: F, t2581: F, t5397: F) -> (F, F, F, F, F, F) {
    let t7293 = t7289 * t7292;
    let t7296 = t2101 * t123;
    let t7297 = t883 * t5263;
    let t7298 = t7296 * t7297;
    let t7299 = t943 * t7298;
    let t7301 = t161 * t2610;
    let t7302 = t2095 * t7301;
    let t7303 = t943 * t7302;
    let t7305 = t2581 * t5397;
    (t7293, t7297, t7299, t7301, t7303, t7305)
}

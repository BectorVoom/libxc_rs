//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 770/1414 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk770<F: Float>(t7373: F, t935: F, t297: F, t313: F, t2748: F, t7371: F, t2672: F) -> (F, F, F, F) {
    let t7374 = t7373 * t935;
    let t7375 = t7374 * t297;
    let t7376 = t313 * t7375;
    let t7379 = t2748 * t7371;
    let t7380 = t2672 * t297;
    (t7375, t7376, t7379, t7380)
}

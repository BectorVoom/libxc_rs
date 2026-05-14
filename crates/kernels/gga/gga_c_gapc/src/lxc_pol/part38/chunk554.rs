//! GGA_C_GAPC lxc pol — lxc_pol part 38 (v4rho2sigma2_17) CSE chunk 554/1126 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part38_v4rho2sigma2_17_chunk554<F: Float>(t3430: F, t3431: F, t1026: F, t928: F, t334: F, t19: F, t277: F, t3114: F) -> (F, F, F, F, F) {
    let t3432 = t3430 * t3431;
    let t3434 = t928 * t1026;
    let t3435 = t3434 * t334;
    let t3437 = t277 * t19;
    let t3438 = t3437 * t3114;
    (t3432, t3434, t3435, t3437, t3438)
}

//! GGA_C_GAPLOC lxc pol — lxc_pol part 30 (v4rho2sigma2_13) CSE chunk 701/1436 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part30_v4rho2sigma2_13_chunk701<F: Float>(t6341: F, t6422: F, t6482: F, t6549: F, t2353: F, t501: F, t1381: F, t892: F, t1383: F, t921: F, t2497: F, t605: F) -> (F, F, F, F, F) {
    let t6551 = t6341 + t6422 + t6482 + t6549;
    let t6553 = t2353 * t501;
    let t6556 = t892 * t1381;
    let t6565 = t921 * t1383;
    let t6568 = t2497 * t605;
    (t6551, t6553, t6556, t6565, t6568)
}

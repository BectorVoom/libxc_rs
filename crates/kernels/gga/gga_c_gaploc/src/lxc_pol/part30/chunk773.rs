//! GGA_C_GAPLOC lxc pol — lxc_pol part 30 (v4rho2sigma2_13) CSE chunk 773/1268 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part30_v4rho2sigma2_13_chunk773<F: Float>(t2748: F, t471: F, t6363: F, t6366: F, t6379: F, t6381: F, t64: F, t7851: F, t90: F, t984: F) -> (F,) {
    let t7861 = t7851 * t471 - 8.0 / 3.0 * t2748 * t64 + 4.0 / 3.0 * t984 * t90 + 63.0 / 256.0 * t6363 - 49.0 / 8192.0 * t6366 + 49.0 / 24576.0 * t6379 - 21.0 / 256.0 * t6381;
    (t7861,)
}

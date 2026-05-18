//! GGA_C_GAPC lxc pol — lxc_pol part 36 (v4rho2sigma2_15) CSE chunk 677/1328 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part36_v4rho2sigma2_15_chunk677<F: Float>(t2468: F, t880: F, t882: F, t319: F, t2206: F, t311: F) -> (F, F, F, F, F) {
    let t7056 = t880 * t2468;
    let t7061 = t882 * t882;
    let t7062 = F::new(1.0) / t7061;
    let t7063 = t319 * t7062;
    let t7073 = t311 * t2206;
    (t7056, t7061, t7062, t7063, t7073)
}

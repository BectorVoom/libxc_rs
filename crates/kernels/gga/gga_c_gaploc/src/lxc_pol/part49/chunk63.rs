//! GGA_C_GAPLOC lxc pol — lxc_pol part 49 (v4rhosigma3_14) CSE chunk 63/1217 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part49_v4rhosigma3_14_chunk63<F: Float>(t64: F, t80: F, t87: F, t90: F, t99: F, t257: F, t260: F) -> (F, F) {
    let t266 = -F::new(0.77371026992393176896e-2) * t64 + F::new(0.187495875e-2) * t80 - F::new(0.362780625e-3) * t87 + F::new(0.10208501871552144532e-4) * t90 - F::new(0.8659659375e-6) * t99;
    let t268 = F::new(0.10636476373080147432e-2) * t64 * t257 - t260 * t266;
    (t266, t268)
}

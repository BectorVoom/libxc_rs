//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 2239/2357 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2239<F: Float>(t16314: F, t26309: F, t16227: F, t22833: F, t1339: F, t57643: F, t6936: F, t22827: F, t550: F, t56805: F, t54165: F, t16060: F, t6944: F) -> (F, F, F, F, F, F) {
    let t91261 = t26309 * t16314;
    let t91263 = t22833 * t16227;
    let t91268 = t6936 * t1339 * t57643;
    let t91272 = t22827 * t1339 * t56805 * t550;
    let t91276 = t22827 * t1339 * t54165 * t550;
    let t91278 = t16060 * t6944;
    (t91261, t91263, t91268, t91272, t91276, t91278)
}

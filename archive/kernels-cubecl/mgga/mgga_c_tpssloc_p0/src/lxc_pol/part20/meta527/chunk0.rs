//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2061/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2061<F: Float>(t1369: F, t40059: F, t12345: F, t3876: F, t22843: F, t241: F, t67: F, t3872: F, t12353: F, t3866: F, t12339: F, t12211: F, t12375: F) -> (F, F, F, F, F, F, F) {
    let t40060 = t40059 * t1369;
    let t40065 = t12345 * t3876;
    let t40070 = t241 * t22843 * t67;
    let t40079 = t12345 * t3872;
    let t40081 = t3866 * t12353;
    let t40083 = t12339 * t3872;
    let t40089 = t12211 * t12375;
    (t40060, t40065, t40070, t40079, t40081, t40083, t40089)
}

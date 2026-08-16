//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2042/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2042<F: Float>(t1369: F, t40059: F, t22843: F, t241: F, t67: F, t10021: F, t1336: F, t1339: F, t1354: F, t12384: F, t3777: F, t12282: F) -> (F, F, F, F, F, F) {
    let t40060 = t40059 * t1369;
    let t40070 = t241 * t22843 * t67;
    let t40123 = t1336 * t1339 * t10021;
    let t40124 = t40123 * t1354;
    let t40130 = t3777 * t12384;
    let t40138 = t3777 * t12282;
    (t40060, t40070, t40123, t40124, t40130, t40138)
}

//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2186/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2186<F: Float>(t40159: F, t6390: F, t19815: F, t3798: F, t1354: F, t40130: F, t1827: F, t54532: F, t16232: F, t5234: F, t1351: F, t6387: F) -> (F, F, F, F, F, F, F) {
    let t57041 = t40159 * t6390;
    let t57056 = t19815 * t3798;
    let t57057 = t57056 * t1354;
    let t57071 = t40130 * t6390;
    let t57073 = t54532 * t1827;
    let t57081 = t5234 * t16232;
    let t57091 = t6387 * t1351;
    (t57041, t57056, t57057, t57071, t57073, t57081, t57091)
}

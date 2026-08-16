//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2643/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2643<F: Float>(t1336: F, t2691: F, t3788: F, t5252: F, t16028: F, t225: F, t40041: F, t544: F, t68: F, t1332: F, t16046: F, t1338: F, t16413: F) -> (F, F, F, F, F) {
    let t54811 = t1336 * t3788 * t2691 * t5252;
    let t54825 = t16028 * t225;
    let t54963 = t544 * t68 * t40041;
    let t54976 = t1332 * t16046;
    let t55039 = t1338 * t16413;
    (t54811, t54825, t54963, t54976, t55039)
}

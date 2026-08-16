//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2627/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2627<F: Float>(t12283: F, t16248: F, t40138: F, t5293: F, t16275: F, t16271: F, t16383: F, t16370: F, t16060: F, t3798: F, t1354: F, t12345: F, t5310: F) -> (F, F, F, F, F, F, F, F, F) {
    let t54088 = t12283 * t16248;
    let t54090 = t40138 * t5293;
    let t54092 = t12283 * t16275;
    let t54114 = t12283 * t16271;
    let t54116 = t12283 * t16383;
    let t54118 = t12283 * t16370;
    let t54124 = t16060 * t3798;
    let t54125 = t54124 * t1354;
    let t54131 = t12345 * t5310;
    (t54088, t54090, t54092, t54114, t54116, t54118, t54124, t54125, t54131)
}

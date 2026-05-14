//! MGGA_C_R2SCAN lxc pol — lxc_pol part 6 (v4rho4_1) CSE chunk 1292/1462 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part6_v4rho4_1_chunk1292<F: Float>(t2533: F, t7944: F, t495: F, t8048: F, t22744: F, t7323: F, t1551: F, t7604: F, t22709: F, t6583: F, t7326: F, t20454: F, t7338: F, t5108: F, t7963: F, t6132: F, t7345: F) -> (F, F, F, F, F, F, F, F) {
    let t24288 = t2533 * t7944;
    let t24292 = t8048 * t495;
    let t24298 = t22744 * t7323;
    let t24300 = t7604 * t1551;
    let t24305 = t6583 * t22709 * t7326;
    let t24318 = t7338 * t20454;
    let t24323 = t5108 * t22709 * t7963;
    let t24326 = t6132 * t22709 * t7345;
    (t24288, t24292, t24298, t24300, t24305, t24318, t24323, t24326)
}

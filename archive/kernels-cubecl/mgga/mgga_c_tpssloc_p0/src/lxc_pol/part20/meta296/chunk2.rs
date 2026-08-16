//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 1513/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1513<F: Float>(t3196: F, t4684: F, t11007: F, t383: F, t1014: F, t10471: F, t10470: F) -> (F, F, F, F) {
    let t11040 = t3196 * t4684;
    let t11043 = t383 * t11007;
    let t11045 = t10471 * t1014;
    let t11046 = t10470 * t11045;
    (t11040, t11043, t11045, t11046)
}

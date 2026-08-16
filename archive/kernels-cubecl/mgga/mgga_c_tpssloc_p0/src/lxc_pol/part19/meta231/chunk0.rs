//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 938/1497 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk938<F: Float>(t11007: F, t383: F, t1014: F, t10471: F, t10470: F, t10481: F, t381: F, t360: F, t6739: F, t1057: F, t10960: F, t3120: F, t3188: F) -> (F, F, F, F, F, F, F, F) {
    let t11043 = t383 * t11007;
    let t11045 = t10471 * t1014;
    let t11046 = t10470 * t11045;
    let t11047 = t381 * t10481;
    let t11048 = t6739 * t360;
    let t11049 = t11047 * t11048;
    let t11051 = t10960 * t1057;
    let t11054 = t3188 * t3120;
    (t11043, t11045, t11046, t11047, t11048, t11049, t11051, t11054)
}

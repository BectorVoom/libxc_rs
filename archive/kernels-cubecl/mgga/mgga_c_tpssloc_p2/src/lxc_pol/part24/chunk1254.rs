//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 24 (v4rho3sigma_0) CSE chunk 1254/1438 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part24_v4rho3sigma_0_chunk1254<F: Float>(t22705: F, t22852: F, t236: F, t3850: F, t550: F, t12238: F, t2002: F, t559: F, t1361: F, t22690: F, t22792: F, t3719: F) -> (F, F, F) {
    let t80807 = t22852 * t22705 * t236 * t3850 * t550;
    let t80810 = t12238 * t2002 * t559;
    let t80814 = t22792 * t22690 * t1361 * t3719;
    (t80807, t80810, t80814)
}

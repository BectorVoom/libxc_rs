//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1134/1340 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1134<F: Float>(t179: F, t18199: F, t299: F, t3542: F, t2099: F, t2945: F, t9590: F, t154: F, t2048: F, t276: F, t9161: F, t300: F, t3638: F, t779: F) -> (F, F, F, F) {
    let t25248 = t299 * t179 * t18199 * t3542;
    let t25275 = t2945 * t2099 * t9590;
    let t25290 = t276 * t154 * t2048 * t9161;
    let t25337 = t300 * t779 * t3638;
    (t25248, t25275, t25290, t25337)
}

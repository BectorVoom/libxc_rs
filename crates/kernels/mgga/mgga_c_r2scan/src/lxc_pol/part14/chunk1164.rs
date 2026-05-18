//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 1164/1276 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk1164<F: Float>(t3578: F, t494: F, t97: F, t113: F, t11505: F, t1543: F, t2867: F, t2259: F, t3582: F, t3446: F, t3453: F, t7133: F) -> (F, F, F, F, F) {
    let t40276 = t97 * t3578 * t494;
    let t40282 = t97 * t11505 * t113;
    let t40285 = t2867 * t1543;
    let t40289 = t3582 * t2259;
    let t40294 = t3446 * t3453 * t7133;
    (t40276, t40282, t40285, t40289, t40294)
}

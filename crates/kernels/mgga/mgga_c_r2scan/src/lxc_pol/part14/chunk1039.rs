//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 1039/1124 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk1039<F: Float>(t1543: F, t2867: F, t2259: F, t3582: F, t3446: F, t3453: F, t7133: F, t795: F, t983: F, t481: F, t11588: F, t38355: F, t11592: F, t37400: F, t10680: F, t11587: F, t37421: F) -> (F, F, F, F, F, F, F, F) {
    let t40285 = t2867 * t1543;
    let t40289 = t3582 * t2259;
    let t40294 = t3446 * t3453 * t7133;
    let t40296 = t983 * t795;
    let t40297 = t40296 * t481;
    let t40303 = t38355 * t11588;
    let t40305 = t37400 * t11592;
    let t40308 = t10680 * t11587 * t37421;
    (t40285, t40289, t40294, t40296, t40297, t40303, t40305, t40308)
}

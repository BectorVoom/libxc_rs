//! MGGA_C_R2SCAN lxc pol — lxc_pol part 13 (v4rho3sigma_3) CSE chunk 1191/1253 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part13_v4rho3sigma_3_chunk1191<F: Float>(t10997: F, t3275: F, t40289: F, t3446: F, t3453: F, t7133: F, t795: F, t983: F, t481: F, t37327: F, t4176: F, t11487: F, t37282: F) -> (F, F, F, F, F) {
    let t40292 = F::cast_from(45.0_f64) / F::cast_from(64.0_f64) * t3275 * t10997 * t40289;
    let t40294 = t3446 * t3453 * t7133;
    let t40296 = t983 * t795;
    let t40297 = t40296 * t481;
    let t40300 = F::cast_from(15.0_f64) / F::cast_from(8.0_f64) * t37327 * t4176 * t40297;
    let t40302 = F::cast_from(15.0_f64) / F::cast_from(8.0_f64) * t37282 * t11487;
    (t40292, t40294, t40296, t40300, t40302)
}

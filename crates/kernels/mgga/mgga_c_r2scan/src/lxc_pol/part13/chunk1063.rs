//! MGGA_C_R2SCAN lxc pol — lxc_pol part 13 (v4rho3sigma_3) CSE chunk 1063/1115 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part13_v4rho3sigma_3_chunk1063<F: Float>(t795: F, t983: F, t481: F, t37327: F, t4176: F, t11487: F, t37282: F, t11588: F, t38355: F, t11592: F, t37400: F, t10680: F, t11587: F, t37421: F, t2768: F, t874: F) -> (F, F, F, F, F, F, F) {
    let t40296 = t983 * t795;
    let t40297 = t40296 * t481;
    let t40300 = 15.0 / 8.0 * t37327 * t4176 * t40297;
    let t40302 = 15.0 / 8.0 * t37282 * t11487;
    let t40303 = t38355 * t11588;
    let t40305 = t37400 * t11592;
    let t40308 = t10680 * t11587 * t37421;
    let t40310 = t2768 * t874;
    (t40296, t40300, t40302, t40303, t40305, t40308, t40310)
}

//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 1168/1276 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk1168<F: Float>(t7088: F, t797: F, t114: F, t1543: F, t97: F, t481: F, t7040: F, t3446: F, t3453: F, t7098: F, t7101: F, t104: F, t920: F) -> (F, F, F, F, F, F) {
    let t40374 = t797 * t7088;
    let t40379 = t97 * t1543 * t114;
    let t40383 = t7040 * t481;
    let t40388 = t3446 * t3453 * t7098;
    let t40391 = t3446 * t3453 * t7101;
    let t40393 = t104 * t920;
    (t40374, t40379, t40383, t40388, t40391, t40393)
}

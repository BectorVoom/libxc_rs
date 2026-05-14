//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 1041/1124 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk1041<F: Float>(t10673: F, t10674: F, t10676: F, t2482: F, t106: F, t7194: F, t97: F, t7088: F, t797: F, t114: F, t1543: F, t481: F, t7040: F, t3446: F, t3453: F, t7098: F) -> (F, F, F, F, F, F) {
    let t40345 = t10673 * t10674 * t2482 * t10676;
    let t40358 = t97 * t106 * t7194;
    let t40374 = t797 * t7088;
    let t40379 = t97 * t1543 * t114;
    let t40383 = t7040 * t481;
    let t40388 = t3446 * t3453 * t7098;
    (t40345, t40358, t40374, t40379, t40383, t40388)
}

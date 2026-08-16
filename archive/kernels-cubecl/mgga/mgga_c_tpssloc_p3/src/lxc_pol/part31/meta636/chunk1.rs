//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 1902/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1902<F: Float>(t6888: F, t6891: F, t97511: F, t22633: F, t28116: F, t80650: F, t1808: F, t254: F, t1377: F, t6347: F, t1385: F, t22635: F) -> (F, F, F, F) {
    let t97619 = t6888 * t97511 * t6891;
    let t97624 = t22633 * t80650 * t28116;
    let t97626 = t1808 * t254;
    let t97637 = t1377 * t6347;
    let t97640 = t22633 * t22635 * t97637 * t1385;
    (t97619, t97624, t97626, t97640)
}

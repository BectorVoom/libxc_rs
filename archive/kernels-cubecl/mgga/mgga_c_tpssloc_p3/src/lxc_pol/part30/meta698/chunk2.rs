//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 2239/2341 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2239<F: Float>(t22690: F, t23122: F, t5544: F, t841: F, t23097: F, t5617: F, t776: F, t815: F, t1510: F, t4233: F, t6605: F, t232: F, t58688: F) -> (F, F, F, F) {
    let t98647 = t23122 * t22690 * t841 * t5544;
    let t98651 = t23097 * t815 * t5617 * t776;
    let t98655 = t6605 * t815 * t1510 * t4233;
    let t98659 = t6605 * t815 * t58688 * t232;
    (t98647, t98651, t98655, t98659)
}

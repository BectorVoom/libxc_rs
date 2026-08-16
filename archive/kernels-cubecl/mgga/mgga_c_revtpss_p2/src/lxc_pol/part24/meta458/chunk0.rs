//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1428/1850 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1428<F: Float>(t1086: F, t15669: F, t3090: F, t11629: F, t53703: F, t3316: F, t4746: F, t4891: F, t1025: F, t1663: F, t2434: F, t371: F) -> (F, F, F, F) {
    let t54500 = t15669 * t1086 * t3090;
    let t54564 = t53703 * t11629;
    let t54570 = t4746 * t3316 * t4891;
    let t54687 = t1025 * t371 * t2434 * t1663;
    (t54500, t54564, t54570, t54687)
}

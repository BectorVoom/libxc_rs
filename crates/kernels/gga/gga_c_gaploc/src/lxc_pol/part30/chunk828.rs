//! GGA_C_GAPLOC lxc pol — lxc_pol part 30 (v4rho2sigma2_13) CSE chunk 828/1268 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part30_v4rho2sigma2_13_chunk828<F: Float>(t8466: F, t8524: F, t8573: F, t8626: F, t8688: F, t8760: F, t8796: F, t8851: F, t3073: F, t841: F, t1033: F, t1959: F, t161: F, t2931: F, t1854: F, t1858: F, t3487: F) -> (F, F, F, F, F) {
    let t8854 = t8466 + t8524 + t8573 + t8626 + t8688 + t8760 + t8796 + t8851;
    let t8859 = t3073 * t841;
    let t8862 = t1033 * t1959;
    let t8867 = t2931 * t161;
    let t8868 = t8867 * t1854;
    let t8871 = t1858 * t3487;
    (t8854, t8859, t8862, t8868, t8871)
}

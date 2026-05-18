//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 657/1340 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk657<F: Float>(t3747: F, t834: F, t2215: F, t3743: F, t841: F, t1167: F) -> (F, F, F, F) {
    let t3748 = t834 * t3747;
    let t3752 = t2215 * t3743;
    let t3754 = t841 * t3747;
    let t3757 = t1167 * t1167;
    (t3748, t3752, t3754, t3757)
}

//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3110/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3110<F: Float>(t1063: F, t15833: F, t3172: F, t11779: F, t4845: F, t15749: F, t3211: F, t16148: F, t4837: F, t11656: F, t15769: F, t16199: F, t372: F) -> (F, F, F, F, F, F) {
    let t54638 = t1063 * t3172 * t15833;
    let t54646 = t11779 * t4845;
    let t54648 = t3211 * t15749;
    let t54651 = t4837 * t3172 * t16148;
    let t54656 = t11656 * t15769;
    let t54658 = t372 * t16199;
    (t54638, t54646, t54648, t54651, t54656, t54658)
}

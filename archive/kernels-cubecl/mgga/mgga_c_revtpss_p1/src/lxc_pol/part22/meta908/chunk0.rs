//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3109/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3109<F: Float>(t11710: F, t15958: F, t3091: F, t3316: F, t4746: F, t4891: F, t16381: F, t3090: F, t11262: F, t3127: F, t4874: F, t15758: F, t16055: F) -> (F, F, F, F, F) {
    let t54553 = t3091 * t11710 * t15958;
    let t54570 = t4746 * t3316 * t4891;
    let t54578 = t16381 * t3090;
    let t54599 = t3127 * t11262 * t4874;
    let t54623 = t15758 * t16055;
    (t54553, t54570, t54578, t54599, t54623)
}

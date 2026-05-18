//! GGA_C_GAPC lxc pol — lxc_pol part 29 (v4rho2sigma2_8) CSE chunk 1113/1311 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part29_v4rho2sigma2_8_chunk1113<F: Float>(t125: F, t24760: F, t24132: F, t277: F, t28416: F, t11755: F, t641: F, t761: F, t3775: F, t9599: F, t11913: F, t29228: F) -> (F, F, F, F, F) {
    let t33781 = t24760 * t125;
    let t33784 = t277 * t33781 * t24132 * t28416;
    let t33787 = t761 * t641 * t11755;
    let t33789 = t3775 * t9599;
    let t33791 = t11913 * t29228;
    (t33781, t33784, t33787, t33789, t33791)
}

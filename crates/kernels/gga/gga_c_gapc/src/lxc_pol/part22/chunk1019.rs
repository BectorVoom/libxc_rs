//! GGA_C_GAPC lxc pol — lxc_pol part 22 (v4rho2sigma2_1) CSE chunk 1019/1209 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part22_v4rho2sigma2_1_chunk1019<F: Float>(t11755: F, t641: F, t761: F, t3775: F, t9599: F, t11913: F, t29228: F, t11872: F, t9990: F, t11356: F, t28472: F, t9574: F, t33152: F, t3402: F, t9934: F, t28924: F) -> (F, F, F, F, F, F, F) {
    let t33787 = t761 * t641 * t11755;
    let t33789 = t3775 * t9599;
    let t33791 = t11913 * t29228;
    let t33793 = t11872 * t9990;
    let t33796 = t9574 * t11356 * t28472;
    let t33801 = t3402 * t33152 * t9934;
    let t33803 = t11913 * t28924;
    (t33787, t33789, t33791, t33793, t33796, t33801, t33803)
}

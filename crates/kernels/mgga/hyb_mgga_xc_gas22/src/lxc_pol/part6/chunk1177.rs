//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1177/1345 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1177<F: Float>(t26727: F, t2868: F, t10154: F, t10516: F, t11261: F, t11263: F, t11265: F, t8586: F, t8951: F, t8953: F, t9315: F, t9316: F, t9412: F, t9413: F, t9415: F, t3823: F, t555: F, t6160: F) -> (F, F, F) {
    let t26976 = t2868 * t26727;
    let t27002 = 2.0 * t10154 + 2.0 * t10516 + 2.0 * t9315 + 2.0 * t9316 + 2.0 * t9412 + 4.0 * t8951 + 2.0 * t8953 + 4.0 * t9413 + 2.0 * t9415 + 4.0 * t11263 + 2.0 * t11265 + 2.0 * t8586 + 4.0 * t11261;
    let t27007 = t555 * t6160 * t3823;
    (t26976, t27002, t27007)
}

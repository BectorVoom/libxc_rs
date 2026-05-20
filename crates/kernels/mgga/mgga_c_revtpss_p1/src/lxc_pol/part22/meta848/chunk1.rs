//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 2988/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2988<F: Float>(t2782: F, t4077: F, t47794: F, t556: F, t1426: F, t5711: F, t786: F, t3917: F, t3899: F, t5775: F, t689: F, t14100: F, t9686: F) -> (F, F, F, F, F) {
    let t49497 = t2782 * t556 * t47794 * t4077;
    let t49503 = t786 * t5711 * t1426;
    let t49504 = t49503 * t3917;
    let t49508 = t689 * t3899 * t5775;
    let t49512 = t14100 * t9686;
    (t49497, t49503, t49504, t49508, t49512)
}

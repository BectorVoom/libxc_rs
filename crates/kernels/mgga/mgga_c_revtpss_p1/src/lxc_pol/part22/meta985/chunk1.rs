//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3336/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3336<F: Float>(t4606: F, t918: F, t15107: F, t15110: F, t128: F, t63248: F, t904: F) -> (F, F, F) {
    let t63265 = t918 * t4606;
    let t63266 = t15107 * t63265;
    let t63268 = t15110 * t63265;
    let t63274 = t128 * t904 * t63248;
    (t63266, t63268, t63274)
}

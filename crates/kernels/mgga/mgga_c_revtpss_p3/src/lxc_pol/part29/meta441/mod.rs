//! MGGA_C_REVTPSS lxc pol kernel — _part29_v4rho3sigma_4 meta441 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1653;
use chunk1::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1654;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_meta441<F: Float>(t14365: F, t25207: F, t605: F, t775: F, t2430: F, t30: F, t1946: F, t2684: F, t7043: F, t820: F, t843: F, t857: F, t2656: F, t7045: F, t240: F, t7036: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t25208, t25211, t25215, t25219, t25222) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1653::<F>(t14365, t25207, t605, t775, t2430, t30, t1946, t2684, t7043, t820, t843);
        let (t25223, t25224, t25225, t25227) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1654::<F>(t25222, t857, t2656, t7045, t240, t7036);
    (t25208, t25211, t25215, t25219, t25222, t25223, t25224, t25225, t25227)
}

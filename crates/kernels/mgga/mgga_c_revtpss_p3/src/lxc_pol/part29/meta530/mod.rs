//! MGGA_C_REVTPSS lxc pol kernel — _part29_v4rho3sigma_4 meta530 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1859;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_meta530<F: Float>(t25120: F, t7349: F, t2247: F, t239: F, t38: F, t6960: F, t25163: F, t7348: F, t25162: F, t2047: F, t92576: F, t92584: F) -> (F, F, F, F, F, F, F) {
        let (t95290, t95293, t95294, t95296, t95297, t95303, t95306) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1859::<F>(t25120, t7349, t2247, t239, t38, t6960, t25163, t7348, t25162, t2047, t92576, t92584);
    (t95290, t95293, t95294, t95296, t95297, t95303, t95306)
}

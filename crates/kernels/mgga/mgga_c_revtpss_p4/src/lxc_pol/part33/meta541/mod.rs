//! MGGA_C_REVTPSS lxc pol kernel — _part33_v4rho3sigma_8 meta541 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1912;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_meta541<F: Float>(t225: F, t29109: F, t494: F, t1769: F, t7627: F, t7637: F, t11239: F, t1276: F, t3596: F, t2149: F, t29157: F, t3153: F) -> (F, F, F, F, F, F, F) {
        let (t29183, t29186, t29187, t29192, t29193, t29194, t29195) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1912::<F>(t225, t29109, t494, t1769, t7627, t7637, t11239, t1276, t3596, t2149, t29157, t3153);
    (t29183, t29186, t29187, t29192, t29193, t29194, t29195)
}

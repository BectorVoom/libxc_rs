//! MGGA_C_REVTPSS lxc pol kernel — _part33_v4rho3sigma_8 meta585 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1998;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_meta585<F: Float>(t2434: F, t837: F, t25377: F, t25431: F, t251: F, t25304: F, t25374: F, t10505: F, t93172: F, t2453: F, t25398: F, t10506: F) -> (F, F, F, F, F, F, F, F) {
        let (t93183, t93184, t93189, t93190, t93191, t93192, t93194, t93195) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1998::<F>(t2434, t837, t25377, t25431, t251, t25304, t25374, t10505, t93172, t2453, t25398, t10506);
    (t93183, t93184, t93189, t93190, t93191, t93192, t93194, t93195)
}

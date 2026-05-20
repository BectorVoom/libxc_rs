//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta993 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3379;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3380;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta993<F: Float>(t141: F, t63297: F, t930: F, t2908: F, t63364: F, t63283: F, t11341: F, t63288: F, t63449: F, t2439: F, t6132: F, t63455: F, t6135: F, t52126: F, t52128: F, t63447: F, t63451: F, t63453: F, t63457: F, t63459: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t63519, t63522, t63525, t63528, t63531, t63533, t63536) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3379::<F>(t141, t63297, t930, t2908, t63364, t63283, t11341, t63288, t63449, t2439, t6132, t63455);
        let (t63538, t63540) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3380::<F>(t2439, t6135, t52126, t52128, t63447, t63451, t63453, t63457, t63459, t63519, t63522, t63525, t63528, t63531, t63533, t63536);
    (t63519, t63522, t63525, t63528, t63531, t63533, t63536, t63538, t63540)
}

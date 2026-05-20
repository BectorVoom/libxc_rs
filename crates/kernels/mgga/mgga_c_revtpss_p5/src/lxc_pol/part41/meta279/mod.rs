//! MGGA_C_REVTPSS lxc pol kernel — _part41_v4rho3tau_4 meta279 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1031;
use chunk1::mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1032;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_meta279<F: Float>(t200: F, t45: F, t202: F, t57: F, t2435: F, t2445: F, t2441: F, t9303: F, t10115: F, t258: F, t2453: F, t2464: F, t2438: F, t886: F, t138: F, t2434: F, t123: F, t2465: F, t215: F, t231: F, t268: F, t836: F, t2798: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t10446, t10457, t10498, t10501, t10503, t10504) = mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1031::<F>(t200, t45, t202, t57, t2435, t2445, t2441, t9303, t10115, t258, t2453, t2464);
        let (t10507, t10511, t10519) = mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1032::<F>(t2438, t886, t138, t10504, t2434, t123, t2465, t215, t231, t268, t836, t2798);
    (t10446, t10457, t10498, t10501, t10503, t10504, t10507, t10511, t10519)
}

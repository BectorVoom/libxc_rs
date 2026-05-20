//! MGGA_C_REVTPSS lxc pol kernel — _part39_v4rho3tau_2 meta146 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk681;
use chunk1::mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk682;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_meta146<F: Float>(t225: F, t3259: F, t385: F, t1071: F, t342: F, t1077: F, t384: F, t1096: F, t1086: F, t989: F) -> (F, F, F, F, F, F, F) {
        let (t3261, t3264, t3268, t3269) = mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk681::<F>(t225, t3259, t385, t1071, t342, t1077, t384);
        let (t3270, t3271, t3278) = mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk682::<F>(t1096, t3269, t1086, t989);
    (t3261, t3264, t3268, t3269, t3270, t3271, t3278)
}

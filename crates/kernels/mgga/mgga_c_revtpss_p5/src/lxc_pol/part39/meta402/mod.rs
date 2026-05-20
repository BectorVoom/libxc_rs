//! MGGA_C_REVTPSS lxc pol kernel — _part39_v4rho3tau_2 meta402 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1474;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_meta402<F: Float>(t31032: F, t8269: F, t10208: F, t69: F, t2340: F, t8259: F, t101: F, t43: F, t665: F, t658: F, t8268: F, t2366: F) -> (F, F, F, F, F, F, F, F) {
        let (t31033, t31035, t31036, t31039, t31040, t31043, t31044, t31047) = mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1474::<F>(t31032, t8269, t10208, t69, t2340, t8259, t101, t43, t665, t658, t8268, t2366);
    (t31033, t31035, t31036, t31039, t31040, t31043, t31044, t31047)
}

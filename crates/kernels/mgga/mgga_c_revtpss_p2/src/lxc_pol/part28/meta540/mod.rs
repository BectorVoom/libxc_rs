//! MGGA_C_REVTPSS lxc pol kernel — _part28_v4rho3sigma_3 meta540 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1989;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_meta540<F: Float>(t4147: F, t7311: F, t1925: F, t36: F, t606: F, t7933: F, t1450: F, t11239: F, t3268: F, t211: F, t9644: F, t138: F, t785: F, t9302: F) -> (F, F, F, F, F, F, F) {
        let (t32113, t32592, t33651, t35070, t36870, t39643, t40270) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1989::<F>(t4147, t7311, t1925, t36, t606, t7933, t1450, t11239, t3268, t211, t9644, t138, t785, t9302);
    (t32113, t32592, t33651, t35070, t36870, t39643, t40270)
}

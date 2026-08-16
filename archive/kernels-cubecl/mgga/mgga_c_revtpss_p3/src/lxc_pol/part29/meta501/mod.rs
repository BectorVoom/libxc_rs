//! MGGA_C_REVTPSS lxc pol kernel — _part29_v4rho3sigma_4 meta501 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1818;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_meta501<F: Float>(t4147: F, t7535: F, t36: F, t68: F, t606: F, t8107: F, t1450: F, t211: F, t9644: F, t138: F, t785: F, t9302: F) -> (F, F, F, F, F, F) {
        let (t33183, t33269, t34495, t35312, t39643, t40270) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1818::<F>(t4147, t7535, t36, t68, t606, t8107, t1450, t211, t9644, t138, t785, t9302);
    (t33183, t33269, t34495, t35312, t39643, t40270)
}

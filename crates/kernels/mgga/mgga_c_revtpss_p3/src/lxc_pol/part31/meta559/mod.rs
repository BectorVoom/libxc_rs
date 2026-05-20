//! MGGA_C_REVTPSS lxc pol kernel — _part31_v4rho3sigma_6 meta559 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1969;
use chunk1::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1970;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_meta559<F: Float>(t211: F, t9644: F, t138: F, t785: F, t9302: F, t2452: F, t9720: F, t11006: F, t256: F, t10115: F, t251: F, t2410: F, t3335: F, t11198: F, t340: F, t11119: F, t384: F, t11238: F, t196: F, t10308: F, t599: F, t90: F, t29: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t39643, t40270, t40688, t41077, t41117, t41153) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1969::<F>(t211, t9644, t138, t785, t9302, t2452, t9720, t11006, t256, t10115, t251, t2410);
        let (t41154, t41937, t42058, t42066, t42859, t45963, t45972) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1970::<F>(t41153, t3335, t11198, t340, t11119, t384, t11238, t196, t10308, t599, t90, t29);
    (t39643, t40270, t40688, t41077, t41117, t41154, t41937, t42058, t42066, t42859, t45963, t45972)
}

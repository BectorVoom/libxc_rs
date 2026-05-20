//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta872 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3033;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3034;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta872<F: Float>(t2475: F, t808: F, t14787: F, t50768: F, t14476: F, t689: F, t887: F, t11028: F, t1580: F, t2439: F, t10504: F, t15002: F, t9285: F, t10505: F, t137: F, t41011: F, t11015: F, t4325: F, t4477: F, t9292: F, t14472: F, t14979: F, t779: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t51176, t51178, t51196, t51199, t51203) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3033::<F>(t2475, t808, t14787, t50768, t14476, t689, t887, t11028, t1580, t2439, t10504, t15002, t9285);
        let (t51207, t51211, t51213, t51216, t51227) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3034::<F>(t10505, t137, t15002, t41011, t11015, t4325, t4477, t9292, t14472, t2439, t887, t14979, t689, t779);
    (t51176, t51178, t51196, t51199, t51203, t51207, t51211, t51213, t51216, t51227)
}

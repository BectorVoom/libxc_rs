//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta901 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3095;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3096;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta901<F: Float>(t16094: F, t53884: F, t11922: F, t16021: F, t4899: F, t3091: F, t43240: F, t4787: F, t12160: F, t15688: F, t1087: F, t43065: F, t3105: F, t4857: F, t1012: F, t43222: F, t16190: F, t3173: F, t15711: F, t3188: F, t1011: F, t15145: F, t15987: F, t15149: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t53885, t53898, t53901, t53914, t53923) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3095::<F>(t16094, t53884, t11922, t16021, t4899, t3091, t43240, t4787, t12160, t15688, t1087, t43065);
        let (t53926, t53944, t53948, t53955, t53958, t53961) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3096::<F>(t3105, t4857, t1012, t43222, t16190, t3173, t15711, t3188, t1011, t15145, t15987, t15149);
    (t53885, t53898, t53901, t53914, t53923, t53926, t53944, t53948, t53955, t53958, t53961)
}

//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta901 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3095;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3096;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta901(t16094: f64, t53884: f64, t11922: f64, t16021: f64, t4899: f64, t3091: f64, t43240: f64, t4787: f64, t12160: f64, t15688: f64, t1087: f64, t43065: f64, t3105: f64, t4857: f64, t1012: f64, t43222: f64, t16190: f64, t3173: f64, t15711: f64, t3188: f64, t1011: f64, t15145: f64, t15987: f64, t15149: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t53885, t53898, t53901, t53914, t53923) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3095(t16094, t53884, t11922, t16021, t4899, t3091, t43240, t4787, t12160, t15688, t1087, t43065);
        let (t53926, t53944, t53948, t53955, t53958, t53961) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3096(t3105, t4857, t1012, t43222, t16190, t3173, t15711, t3188, t1011, t15145, t15987, t15149);
    (t53885, t53898, t53901, t53914, t53923, t53926, t53944, t53948, t53955, t53958, t53961)
}

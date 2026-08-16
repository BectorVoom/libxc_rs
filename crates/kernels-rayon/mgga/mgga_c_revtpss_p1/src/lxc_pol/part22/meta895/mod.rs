//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta895 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3086;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3087;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta895(t247: f64, t42792: f64, t4757: f64, t4837: f64, t15850: f64, t3111: f64, t3091: f64, t43240: f64, t4782: f64, t41296: f64, t42471: f64, t11977: f64, t4820: f64, t1011: f64, t4886: f64, t697: f64, t1065: f64, t372: f64, t4866: f64, t11670: f64, t15904: f64, t12167: f64, t11922: f64, t16081: f64, t16083: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t53431, t53433, t53437, t53473, t53479) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3086(t247, t42792, t4757, t4837, t15850, t3111, t3091, t43240, t4782, t41296, t42471, t11977, t4820);
        let (t53542, t53545, t53552, t53553, t53557) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3087(t1011, t4886, t697, t1065, t372, t4866, t11670, t15904, t12167, t11922, t16081, t16083);
    (t53431, t53433, t53437, t53473, t53479, t53542, t53545, t53552, t53553, t53557)
}

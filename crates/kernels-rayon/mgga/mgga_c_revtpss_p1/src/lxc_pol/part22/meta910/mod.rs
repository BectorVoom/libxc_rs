//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta910 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3113;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3114;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta910(t15731: f64, t3169: f64, t15816: f64, t3168: f64, t11710: f64, t15591: f64, t3091: f64, t16060: f64, t3241: f64, t1011: f64, t140: f64, t16122: f64, t12078: f64, t53740: f64, t12047: f64, t16138: f64, t372: f64, t16158: f64, t3106: f64, t12003: f64, t1659: f64, t11648: f64, t4879: f64, t1063: f64, t15790: f64, t3172: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t54733, t54739, t54785, t54792, t54795) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3113(t15731, t3169, t15816, t3168, t11710, t15591, t3091, t16060, t3241, t1011, t140, t16122);
        let (t54801, t54811, t54818, t54836, t54838, t54841, t54849) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3114(t12078, t53740, t12047, t16138, t372, t16158, t3106, t12003, t1659, t11648, t4879, t1063, t15790, t3172);
    (t54733, t54739, t54785, t54792, t54795, t54801, t54811, t54818, t54836, t54838, t54841, t54849)
}

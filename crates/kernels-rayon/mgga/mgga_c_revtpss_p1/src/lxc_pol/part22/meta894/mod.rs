//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta894 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3083;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3084;
use chunk2::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3085;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta894(t1063: f64, t15193: f64, t247: f64, t3109: f64, t11710: f64, t15600: f64, t3091: f64, t127: f64, t4823: f64, t11774: f64, t3096: f64, t11670: f64, t15687: f64, t3317: f64, t15690: f64, t15689: f64, t15692: f64, t11916: f64, t15932: f64, t11922: f64, t11927: f64, t16026: f64, t15964: f64, t11268: f64, t4820: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t53363, t53389, t53391, t53393, t53401) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3083(t1063, t15193, t247, t3109, t11710, t15600, t3091, t127, t4823, t11774, t3096, t11670, t15687);
        let (t53402, t53405) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3084(t3317, t53401, t127, t15690);
        let (t53407, t53413, t53416, t53422, t53427) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3085(t15689, t15692, t53405, t11916, t15932, t11922, t11927, t16026, t11710, t15964, t3091, t11268, t4820);
    (t53363, t53389, t53391, t53393, t53401, t53402, t53405, t53407, t53413, t53416, t53422, t53427)
}

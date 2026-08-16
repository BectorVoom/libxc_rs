//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta900 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3093;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3094;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta900(t11710: f64, t16089: f64, t16090: f64, t11883: f64, t4924: f64, t1086: f64, t15654: f64, t3090: f64, t11922: f64, t16077: f64, t3115: f64, t225: f64, t53222: f64, t366: f64, t1025: f64, t371: f64, t4852: f64, t676: f64, t53014: f64, t11656: f64, t15734: f64, t11670: f64, t370: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t53820, t53832, t53855, t53859, t53865) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3093(t11710, t16089, t16090, t11883, t4924, t1086, t15654, t3090, t11922, t16077, t3115, t225, t53222);
        let (t53866, t53875, t53877, t53881, t53884) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3094(t366, t53865, t1025, t371, t4852, t676, t225, t53014, t11656, t15734, t11670, t370);
    (t53820, t53832, t53855, t53859, t53865, t53866, t53875, t53877, t53881, t53884)
}

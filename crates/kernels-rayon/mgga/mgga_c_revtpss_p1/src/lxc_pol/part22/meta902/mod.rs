//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta902 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3097;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3098;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta902(t1011: f64, t15154: f64, t15993: f64, t15130: f64, t15135: f64, t11821: f64, t140: f64, t15140: f64, t11710: f64, t15614: f64, t3091: f64, t1063: f64, t15937: f64, t3172: f64, t11672: f64, t15682: f64, t12078: f64, t53552: f64, t16183: f64, t73: f64, t42793: f64, t4892: f64, t4895: f64, t15951: f64, t3127: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t53964, t53967, t53970, t53972, t53974, t53993, t53998) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3097(t1011, t15154, t15993, t15130, t15135, t11821, t140, t15140, t11710, t15614, t3091, t1063, t15937, t3172);
        let (t54014, t54023, t54026, t54036, t54039) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3098(t11672, t15682, t12078, t53552, t16183, t73, t42793, t4892, t4895, t15951, t3127, t3172);
    (t53964, t53967, t53970, t53972, t53974, t53993, t53998, t54014, t54023, t54026, t54036, t54039)
}

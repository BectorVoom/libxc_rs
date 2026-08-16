//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta908 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3109;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3110;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta908(t11710: f64, t15958: f64, t3091: f64, t3316: f64, t4746: f64, t4891: f64, t16381: f64, t3090: f64, t11262: f64, t3127: f64, t4874: f64, t15758: f64, t16055: f64, t1063: f64, t15833: f64, t3172: f64, t11779: f64, t4845: f64, t15749: f64, t3211: f64, t16148: f64, t4837: f64, t11656: f64, t15769: f64, t16199: f64, t372: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t54553, t54570, t54578, t54599, t54623) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3109(t11710, t15958, t3091, t3316, t4746, t4891, t16381, t3090, t11262, t3127, t4874, t15758, t16055);
        let (t54638, t54646, t54648, t54651, t54656, t54658) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3110(t1063, t15833, t3172, t11779, t4845, t15749, t3211, t16148, t4837, t11656, t15769, t16199, t372);
    (t54553, t54570, t54578, t54599, t54623, t54638, t54646, t54648, t54651, t54656, t54658)
}

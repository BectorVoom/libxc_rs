//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta482 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2195;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2196;
use chunk2::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2197;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta482(t11710: f64, t4782: f64, t3091: f64, t1014: f64, t140: f64, t4579: f64, t1011: f64, t11672: f64, t11675: f64, t11881: f64, t11886: f64, t12004: f64, t15952: f64, t15959: f64, t15965: f64, t15970: f64, t15975: f64, t1675: f64, t3127: f64, t4783: f64, t4892: f64, t4899: f64, t3252: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
        let (t15984, t15986, t15987) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2195(t11710, t4782, t3091, t1014, t140);
        let (t15988, t15990, t15991) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2196(t15987, t4579, t1011, t11672, t11675, t11881, t11886, t12004, t15952, t15959, t15965, t15970, t15975, t15986, t1675, t3091, t3127, t4783, t4892, t4899);
        let t15993 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2197(t140, t3252);
    (t15984, t15986, t15987, t15988, t15990, t15991, t15993)
}

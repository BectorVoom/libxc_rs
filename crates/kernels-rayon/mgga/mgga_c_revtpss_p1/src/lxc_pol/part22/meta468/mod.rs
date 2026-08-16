//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta468 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2152;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2153;
use chunk2::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2154;
use chunk3::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2155;
use chunk4::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2156;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta468(t15648: f64, t373: f64, t371: f64, t372: f64, t4742: f64, t993: f64, t225: f64, t366: f64, t3224: f64, t4845: f64, t127: f64, t4852: f64, t1025: f64, t1646: f64, t3056: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t15649, t15651, t15654) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2152(t15648, t373, t371, t372, t4742, t993);
        let t15655 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2153(t15654, t225);
        let t15656 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2154(t15655, t366);
        let (t15662, t15666, t15668, t15669) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2155(t3224, t4845, t127, t371, t4852, t1025, t1646, t3056);
        let t15670 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2156(t15669, t225);
    (t15649, t15651, t15654, t15655, t15656, t15662, t15666, t15668, t15669, t15670)
}

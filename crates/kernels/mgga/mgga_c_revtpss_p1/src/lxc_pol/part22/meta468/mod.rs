//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta468 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2152;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2153;
use chunk2::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2154;
use chunk3::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2155;
use chunk4::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2156;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta468<F: Float>(t15648: F, t373: F, t371: F, t372: F, t4742: F, t993: F, t225: F, t366: F, t3224: F, t4845: F, t127: F, t4852: F, t1025: F, t1646: F, t3056: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t15649, t15651, t15654) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2152::<F>(t15648, t373, t371, t372, t4742, t993);
        let t15655 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2153::<F>(t15654, t225);
        let t15656 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2154::<F>(t15655, t366);
        let (t15662, t15666, t15668, t15669) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2155::<F>(t3224, t4845, t127, t371, t4852, t1025, t1646, t3056);
        let t15670 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2156::<F>(t15669, t225);
    (t15649, t15651, t15654, t15655, t15656, t15662, t15666, t15668, t15669, t15670)
}

//! MGGA_C_REVTPSS lxc pol kernel — _part30_v4rho3sigma_5 meta95 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;
mod chunk7;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk605;
use chunk1::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk606;
use chunk2::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk607;
use chunk3::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk608;
use chunk4::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk609;
use chunk5::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk610;
use chunk6::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk611;
use chunk7::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk612;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_meta95<F: Float>(t2138: F, t467: F, t2134: F, t464: F, t484: F, t225: F, t494: F, t456: F, t1208: F, t487: F, t1032: F, t1276: F, t473: F, t265: F, t502: F, t460: F, t1300: F, t198: F, t1995: F, t336: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t2139, t2142) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk605::<F>(t2138, t467, t2134, t464, t484);
        let (t2143, t2144) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk606::<F>(t2142, t225, t494);
        let (t2147, t2148) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk607::<F>(t456, t1208);
        let t2149 = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk608::<F>(t2148, t487);
        let t2150 = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk609::<F>(t1032, t1276);
        let t2151 = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk610::<F>(t2142, t473);
        let t2152 = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk611::<F>(t2150, t2151);
        let (t2155, t2159) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk612::<F>(t265, t502, t2144, t2149, t2152, t460, t1300, t198, t1995, t336);
    (t2139, t2142, t2143, t2144, t2147, t2148, t2149, t2150, t2151, t2152, t2155, t2159)
}

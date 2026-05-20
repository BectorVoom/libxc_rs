//! MGGA_C_REVTPSS lxc pol kernel — _part33_v4rho3sigma_8 meta93 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk605;
use chunk1::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk606;
use chunk2::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk607;
use chunk3::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk608;
use chunk4::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk609;
use chunk5::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk610;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_meta93<F: Float>(t30: F, t1966: F, t2129: F, t45: F, t343: F, t55: F, t136: F, t473: F, dens_threshold: F, rho0: F, sigma2: F, zeta_threshold: F, t479: F, t467: F, t464: F, t484: F, t225: F, t494: F, t456: F, t1208: F, t487: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t2132, t2133, t2134, t2137) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk605::<F>(t30, t1966, t2129, t45, t343, t55, t136, t473, dens_threshold, rho0, sigma2, zeta_threshold);
        let t2138 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk606::<F>(t2137, t479);
        let (t2139, t2142) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk607::<F>(t2138, t467, t2134, t464, t484);
        let (t2143, t2144) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk608::<F>(t2142, t225, t494);
        let (t2147, t2148) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk609::<F>(t456, t1208);
        let t2149 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk610::<F>(t2148, t487);
    (t2132, t2133, t2134, t2137, t2138, t2139, t2142, t2143, t2144, t2147, t2148, t2149)
}

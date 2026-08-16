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

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk605;
use chunk1::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk606;
use chunk2::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk607;
use chunk3::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk608;
use chunk4::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk609;
use chunk5::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk610;
use chunk6::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk611;
use chunk7::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk612;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_meta95(t2138: f64, t467: f64, t2134: f64, t464: f64, t484: f64, t225: f64, t494: f64, t456: f64, t1208: f64, t487: f64, t1032: f64, t1276: f64, t473: f64, t265: f64, t502: f64, t460: f64, t1300: f64, t198: f64, t1995: f64, t336: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t2139, t2142) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk605(t2138, t467, t2134, t464, t484);
        let (t2143, t2144) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk606(t2142, t225, t494);
        let (t2147, t2148) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk607(t456, t1208);
        let t2149 = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk608(t2148, t487);
        let t2150 = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk609(t1032, t1276);
        let t2151 = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk610(t2142, t473);
        let t2152 = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk611(t2150, t2151);
        let (t2155, t2159) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk612(t265, t502, t2144, t2149, t2152, t460, t1300, t198, t1995, t336);
    (t2139, t2142, t2143, t2144, t2147, t2148, t2149, t2150, t2151, t2152, t2155, t2159)
}

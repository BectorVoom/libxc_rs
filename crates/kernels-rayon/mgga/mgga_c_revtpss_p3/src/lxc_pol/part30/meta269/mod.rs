//! MGGA_C_REVTPSS lxc pol kernel — _part30_v4rho3sigma_5 meta269 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1181;
use chunk1::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1182;
use chunk2::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1183;
use chunk3::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1184;
use chunk4::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1185;
use chunk5::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1186;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_meta269(t670: f64, t7330: f64, t572: f64, t117: f64, t7002: f64, t2121: f64, t38: f64, t2247: f64, t55: f64, t60: f64, t606: f64, t6971: f64, t72: f64, t1927: f64, t2122: f64, t6977: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t7331, t7333, t7334, t7336, t7565) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1181(t670, t7330, t572, t117, t7002, t2121, t38);
        let t7566 = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1182(t2247, t7565);
        let t7571 = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1183(t55, t60);
        let (t7574, t7575) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1184(t606, t6971, t7571, t72);
        let t7576 = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1185(t1927, t7575);
        let t7579 = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1186(t2122, t6977);
    (t7331, t7333, t7334, t7336, t7565, t7566, t7571, t7574, t7575, t7576, t7579)
}

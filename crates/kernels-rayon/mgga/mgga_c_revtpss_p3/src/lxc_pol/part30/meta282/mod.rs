//! MGGA_C_REVTPSS lxc pol kernel — _part30_v4rho3sigma_5 meta282 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1236;
use chunk1::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1237;
use chunk2::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1238;
use chunk3::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1239;
use chunk4::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1240;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_meta282(t30: f64, t1469: f64, t2129: f64, t45: f64, t7794: f64, t8161: f64, t1479: f64, t343: f64, t136: f64, t1785: f64, t2138: f64, t1802: f64, t2137: f64, dens_threshold: f64, rho0: f64, zeta_threshold: f64, t467: f64, t1782: f64, t1791: f64, t1797: f64, t1808: f64, t464: f64, t484: f64, t7606: f64, t7607: f64, t7613: f64, t7618: f64, t7622: f64, t7624: f64, t225: f64, t494: f64, t1769: f64, t2142: f64, t7637: f64, t1774: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t8166, t8171, t8172, t8177, t8184) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1236(t30, t1469, t2129, t45, t7794, t8161, t1479, t343, t136, t1785, t2138, t1802, t2137, dens_threshold, rho0, zeta_threshold);
        let (t8185, t8190) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1237(t467, t8184, t1782, t1791, t1797, t1808, t464, t484, t7606, t7607, t7613, t7618, t7622, t7624, t8172, t8177);
        let (t8192, t8197) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1238(t225, t494, t8190, t1769, t2142);
        let t8198 = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1239(t7637, t8197);
        let t8201 = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1240(t1774, t2142);
    (t8166, t8171, t8172, t8177, t8184, t8185, t8190, t8192, t8197, t8198, t8201)
}

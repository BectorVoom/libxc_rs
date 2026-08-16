//! MGGA_C_REVTPSS lxc pol kernel — _part33_v4rho3sigma_8 meta278 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1238;
use chunk1::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1239;
use chunk2::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1240;
use chunk3::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1241;
use chunk4::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1242;
use chunk5::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1243;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_meta278(t467: f64, t8184: f64, t1782: f64, t1791: f64, t1797: f64, t1808: f64, t464: f64, t484: f64, t7606: f64, t7607: f64, t7613: f64, t7618: f64, t7622: f64, t7624: f64, t8172: f64, t8177: f64, t225: f64, t494: f64, t1769: f64, t2142: f64, t7637: f64, t1774: f64, t1811: f64, t2148: f64, t1828: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t8185, t8190) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1238(t467, t8184, t1782, t1791, t1797, t1808, t464, t484, t7606, t7607, t7613, t7618, t7622, t7624, t8172, t8177);
        let (t8192, t8197) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1239(t225, t494, t8190, t1769, t2142);
        let t8198 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1240(t7637, t8197);
        let t8201 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1241(t1774, t2142);
        let (t8202, t8205) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1242(t7637, t8201, t1811, t2148);
        let t8208 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1243(t1828, t2142);
    (t8185, t8190, t8192, t8197, t8198, t8201, t8202, t8205, t8208)
}

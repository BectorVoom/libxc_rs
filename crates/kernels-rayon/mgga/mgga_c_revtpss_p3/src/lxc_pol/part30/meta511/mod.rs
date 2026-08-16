//! MGGA_C_REVTPSS lxc pol kernel — _part30_v4rho3sigma_5 meta511 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1892;
use chunk1::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1893;
use chunk2::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1894;
use chunk3::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1895;
use chunk4::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1896;
use chunk5::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1897;
use chunk6::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1898;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_meta511(t14756: f64, t27221: f64, t4435: f64, t7045: f64, t4426: f64, t7038: f64, t25245: f64, t4430: f64, t1561: f64, t25266: f64, t25270: f64, t4462: f64, t4447: f64, t4452: f64, t1945: f64, t4371: f64, t807: f64, t25220: f64, t25232: f64, t25246: f64, t25256: f64, t25267: f64, t4458: f64, t7025: f64, t1549: f64, t25277: f64, t4345: f64, t25234: f64, t4349: f64, t25227: f64, t4353: f64, t2661: f64, t1565: f64, t25222: f64, t241: f64, t25260: f64, t820: f64, t4368: f64, t25223: f64, t25229: f64, t25235: f64, t25243: f64, t25254: f64, t25276: f64, t25278: f64, t25284: f64, t233: f64, t1957: f64, t1956: f64, t27183: f64, t27187: f64, t27189: f64, t27192: f64, t27196: f64, t27199: f64, t27203: f64, t27207: f64, t27214: f64, t27217: f64, t4487: f64, t4534: f64, t7053: f64, t7067: f64, t7070: f64, t7073: f64, t7779: f64, t887: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
        let (t27222, t27224, t27226, t27228, t27230, t27232) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1892(t14756, t27221, t4435, t7045, t4426, t7038, t25245, t4430, t1561, t25266, t25270, t4462);
        let (t27239, t27242) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1893(t25270, t4447, t4452, t1945, t4371, t807, t25220, t25232, t25246, t25256, t25267, t27222, t27224, t27226, t27228, t27230, t27232);
        let (t27244, t27246, t27249, t27251, t27253, t27254, t27256) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1894(t4458, t7025, t1549, t25277, t4345, t7045, t25234, t4349, t25227, t4353, t2661, t1565, t25222);
        let t27261 = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1895(t241, t25260, t820);
        let t27264 = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1896(t27261, t4368, t25223, t25229, t25235, t25243, t25254, t25276, t25278, t25284, t27244, t27246, t27249, t27251, t27254, t27256);
        let t27265 = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1897(t27242, t27264);
        let (t27266, t27267, t27272) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1898(t233, t27265, t1957, t1956, t27183, t27187, t27189, t27192, t27196, t27199, t27203, t27207, t27214, t27217, t4487, t4534, t7053, t7067, t7070, t7073, t7779, t887);
    (t27239, t27253, t27261, t27265, t27266, t27267, t27272)
}

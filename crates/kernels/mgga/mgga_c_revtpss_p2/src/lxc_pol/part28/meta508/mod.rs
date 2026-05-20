//! MGGA_C_REVTPSS lxc pol kernel — _part28_v4rho3sigma_3 meta508 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1900;
use chunk1::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1901;
use chunk2::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1902;
use chunk3::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1903;
use chunk4::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1904;
use chunk5::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1905;
use chunk6::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1906;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_meta508<F: Float>(t14756: F, t27221: F, t4435: F, t7045: F, t4426: F, t7038: F, t25245: F, t4430: F, t1561: F, t25266: F, t25270: F, t4462: F, t4447: F, t4452: F, t1945: F, t4371: F, t807: F, t25220: F, t25232: F, t25246: F, t25256: F, t25267: F, t4458: F, t7025: F, t1549: F, t25277: F, t4345: F, t25234: F, t4349: F, t25227: F, t4353: F, t2661: F, t1565: F, t25222: F, t241: F, t25260: F, t820: F, t4368: F, t25223: F, t25229: F, t25235: F, t25243: F, t25254: F, t25276: F, t25278: F, t25284: F, t233: F, t1957: F, t1956: F, t27183: F, t27187: F, t27189: F, t27192: F, t27196: F, t27199: F, t27203: F, t27207: F, t27214: F, t27217: F, t4487: F, t4534: F, t7053: F, t7067: F, t7070: F, t7073: F, t7779: F, t887: F) -> (F, F, F, F, F, F, F) {
        let (t27222, t27224, t27226, t27228, t27230, t27232) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1900::<F>(t14756, t27221, t4435, t7045, t4426, t7038, t25245, t4430, t1561, t25266, t25270, t4462);
        let (t27239, t27242) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1901::<F>(t25270, t4447, t4452, t1945, t4371, t807, t25220, t25232, t25246, t25256, t25267, t27222, t27224, t27226, t27228, t27230, t27232);
        let (t27244, t27246, t27249, t27251, t27253, t27254, t27256) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1902::<F>(t4458, t7025, t1549, t25277, t4345, t7045, t25234, t4349, t25227, t4353, t2661, t1565, t25222);
        let t27261 = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1903::<F>(t241, t25260, t820);
        let t27264 = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1904::<F>(t27261, t4368, t25223, t25229, t25235, t25243, t25254, t25276, t25278, t25284, t27244, t27246, t27249, t27251, t27254, t27256);
        let t27265 = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1905::<F>(t27242, t27264);
        let (t27266, t27267, t27272) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1906::<F>(t233, t27265, t1957, t1956, t27183, t27187, t27189, t27192, t27196, t27199, t27203, t27207, t27214, t27217, t4487, t4534, t7053, t7067, t7070, t7073, t7779, t887);
    (t27239, t27253, t27261, t27265, t27266, t27267, t27272)
}

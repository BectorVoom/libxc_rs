//! MGGA_C_REVTPSS lxc pol kernel — _part31_v4rho3sigma_6 meta270 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1208;
use chunk1::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1209;
use chunk2::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1210;
use chunk3::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1211;
use chunk4::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1212;
use chunk5::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1213;
use chunk6::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1214;
use chunk7::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1215;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_meta270(t1927: f64, t7715: f64, t1493: f64, t76: f64, t1926: f64, t5: f64, t1923: f64, t1928: f64, t6958: f64, t7702: f64, t7706: f64, t7709: f64, t117: f64, t1937: f64, t4248: f64, t1518: f64, t94: f64, t1843: f64, t1936: f64, t114: f64, t651: f64, t1513: f64, t6998: f64, t6997: f64, t508: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let t7716 = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1208(t1927, t7715);
        let t7719 = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1209(t1493, t76);
        let t7720 = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1210(t1926, t7719);
        let (t7724, t7725) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1211(t5, t1923, t1928, t6958, t7702, t7706, t7709, t7716, t7720, t117);
        let (t7731, t7732) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1212(t1937, t4248, t1518, t94);
        let (t7734, t7735) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1213(t1937, t7732, t1843, t1936);
        let (t7737, t7741) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1214(t114, t651, t7735, t1513, t6998, t6997);
        let t7742 = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1215(t508, t7741);
    (t7716, t7719, t7720, t7724, t7725, t7731, t7732, t7734, t7735, t7737, t7741, t7742)
}

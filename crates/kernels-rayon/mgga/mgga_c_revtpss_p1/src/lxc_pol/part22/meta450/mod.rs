//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta450 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2106;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2107;
use chunk2::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2108;
use chunk3::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2109;
use chunk4::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2110;
use chunk5::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2111;
use chunk6::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2112;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta450(t15145: f64, t904: f64, t128: f64, t2258: f64, t4578: f64, t11150: f64, t1469: f64, t2251: f64, t2850: f64, t4573: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t15146, t15147) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2106(t15145, t904, t128);
        let t15149 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2107(t2258, t4578);
        let (t15150, t15151) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2108(t15149, t904, t128);
        let (t15153, t15154) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2109(t11150, t1469, t2251);
        let (t15155, t15156) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2110(t15154, t2850, t128);
        let t15158 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2111(t2251, t4573);
        let (t15159, t15160) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2112(t15158, t904, t128);
    (t15146, t15147, t15149, t15150, t15151, t15153, t15154, t15155, t15156, t15158, t15159, t15160)
}

//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta215 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;
mod chunk7;
mod chunk8;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1354;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1355;
use chunk2::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1356;
use chunk3::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1357;
use chunk4::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1358;
use chunk5::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1359;
use chunk6::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1360;
use chunk7::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1361;
use chunk8::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1362;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta215(t5205: f64, t5206: f64, t1196: f64, t3358: f64, t3546: f64, t5044: f64, t5049: f64, t5054: f64, t5058: f64, t459: f64, t1208: f64, t1769: f64, t487: f64, t1770: f64, t1214: f64, t1774: f64, t1211: f64, t1294: f64, t1277: f64, t3579: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t5207, t5209, t5215, t5216) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1354(t5205, t5206, t1196, t3358, t3546, t5044, t5049, t5054, t5058, t459);
        let t5219 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1355(t1208, t1769);
        let t5220 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1356(t487, t5219);
        let t5225 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1357(t1770, t487);
        let t5230 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1358(t1214, t1774);
        let t5231 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1359(t1211, t5230);
        let t5237 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1360(t1294, t1774, t1277);
        let t5245 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1361(t3358, t3579, t5044, t5049, t5054, t5058);
        let t5246 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1362(t1211, t5245);
    (t5207, t5209, t5215, t5216, t5219, t5220, t5225, t5230, t5231, t5237, t5245, t5246)
}

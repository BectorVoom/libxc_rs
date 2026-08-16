//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta132 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk694;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk695;
use chunk2::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk696;
use chunk3::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk697;
use chunk4::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk698;
use chunk5::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk699;
use chunk6::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk700;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta132(t1263: f64, t1774: f64, t1038: f64, t1802: f64, t1244: f64, t1241: f64, t1121: f64, t3362: f64, t3617: f64, t1012: f64, t1224: f64, t3698: f64, t1234: f64, t1803: f64, t225: f64, t5219: f64, t480: f64, t3623: f64, t4890: f64, t3782: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t5277, t5292, t5293) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk694(t1263, t1774, t1038, t1802, t1244, t1241);
        let (t5296, t5302) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk695(t1121, t1263, t3362, t3617);
        let (t5308, t5312, t5323) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk696(t1012, t1224, t3698, t1234, t1803);
        let t5326 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk697(t225, t5219);
        let t5327 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk698(t480, t5326);
        let t5330 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk699(t3623, t4890);
        let t5331 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk700(t3782, t5330);
    (t5277, t5292, t5293, t5296, t5302, t5308, t5312, t5323, t5326, t5327, t5330, t5331)
}

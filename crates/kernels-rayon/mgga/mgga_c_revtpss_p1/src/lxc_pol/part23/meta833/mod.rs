//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta833 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2696;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2697;
use chunk2::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2698;
use chunk3::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2699;
use chunk4::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2700;
use chunk5::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2701;
use chunk6::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2702;
use chunk7::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2703;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta833(t20294: f64, t689: f64, t20319: f64, t12268: f64, t5825: f64, t20267: f64, t698: f64, t20314: f64, t20303: f64, t20299: f64, t20340: f64, t20377: f64, t20289: f64, t2435: f64, t6426: f64, t20311: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let t68262 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2696(t20294, t689);
        let t68277 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2697(t20319, t689);
        let (t68289, t68312, t68332) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2698(t12268, t5825, t20267, t698, t20314, t689);
        let t68334 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2699(t20303, t689);
        let t68336 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2700(t20299, t689);
        let (t68368, t68370, t68389) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2701(t20340, t698, t20377, t20289, t689);
        let t68399 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2702(t2435, t6426);
        let t68454 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2703(t20311, t689);
    (t68262, t68277, t68289, t68312, t68332, t68334, t68336, t68368, t68370, t68389, t68399, t68454)
}

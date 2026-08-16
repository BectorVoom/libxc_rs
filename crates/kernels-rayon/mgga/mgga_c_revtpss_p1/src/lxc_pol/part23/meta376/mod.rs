//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta376 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1709;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1710;
use chunk2::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1711;
use chunk3::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1712;
use chunk4::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1713;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta376(t11922: f64, t4895: f64, t4892: f64, t140: f64, t4886: f64, t1011: f64, t3241: f64, t4924: f64, t12047: f64, t15905: f64, t12167: f64, t3057: f64, t380: f64, t3088: f64, t370: f64, t994: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t16055, t16057, t16062, t16064, t16067) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1709(t11922, t4895, t4892, t140, t4886, t1011, t3241, t4924, t12047, t15905);
        let t16081 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1710(t12167, t15905);
        let (t16087, t16088) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1711(t3057, t380, t3088, t370);
        let t16089 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1712(t16087, t16088);
        let t16095 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1713(t380, t994, t16088);
    (t16055, t16057, t16062, t16064, t16067, t16081, t16088, t16089, t16095)
}

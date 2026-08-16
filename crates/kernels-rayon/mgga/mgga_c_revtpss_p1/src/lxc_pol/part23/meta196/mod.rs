//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta196 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1181;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1182;
use chunk2::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1183;
use chunk3::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1184;
use chunk4::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1185;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta196(t512: f64, t5567: f64, t1856: f64, t749: f64, t177: f64, t30: f64, t33: f64, t762: f64, t1468: f64, t3874: f64, t1344: f64, t2: f64, t580: f64, t605: f64, t1711: f64, t3881: f64, t1348: f64, t1113: f64, zeta_threshold: f64, t1892: f64, t212: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t5568, t5569) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1181(t512, t5567, t1856, t749);
        let (t5570, t5571) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1182(t512, t5569, t177, t1856);
        let (t5572, t5573, t5574, t5581, t5582, t5589) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1183(t30, t33, t5571, t762, t1468, t3874, t1344, t2, t580, t605, t1711, t3881, t1348, t1113, zeta_threshold);
        let t5591 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1184(t5581, t5589);
        let t5599 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1185(t1892, t212);
    (t5568, t5569, t5570, t5571, t5572, t5573, t5574, t5582, t5591, t5599)
}

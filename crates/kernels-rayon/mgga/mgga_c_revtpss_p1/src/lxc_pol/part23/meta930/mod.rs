//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta930 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3044;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3045;
use chunk2::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3046;
use chunk3::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3047;
use chunk4::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3048;
use chunk5::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3049;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta930(t25026: f64, t3801: f64, t1187: f64, t1756: f64, t58672: f64, t69511: f64, t1130: f64, t24466: f64, t1151: f64, t58339: f64, t6439: f64, t12243: f64, t24221: f64, t1298: f64, t5023: f64, t81128: f64, t81130: f64, t81132: f64, t81134: f64, t81136: f64, t81138: f64, t24237: f64, t689: f64, t24245: f64, t20292: f64, t4186: f64, t12305: f64, t128: f64, t22688: f64, t43776: f64, t606: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t81139, t81145, t81148, t81150, t81152) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3044(t25026, t3801, t1187, t1756, t58672, t69511, t1130, t24466, t1151, t58339, t6439, t12243, t24221);
        let t81153 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3045(t1298, t5023, t81128, t81130, t81132, t81134, t81136, t81138, t81139, t81145, t81148, t81150, t81152);
        let t81156 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3046(t24237, t689);
        let t81158 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3047(t24245, t689);
        let (t81160, t81162) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3048(t20292, t4186, t12305, t128);
        let (t81165, t81167) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3049(t22688, t43776, t606, t12305, t128);
    (t81145, t81148, t81150, t81152, t81153, t81156, t81158, t81160, t81162, t81165, t81167)
}

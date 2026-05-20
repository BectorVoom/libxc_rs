//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta930 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3044;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3045;
use chunk2::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3046;
use chunk3::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3047;
use chunk4::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3048;
use chunk5::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3049;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta930<F: Float>(t25026: F, t3801: F, t1187: F, t1756: F, t58672: F, t69511: F, t1130: F, t24466: F, t1151: F, t58339: F, t6439: F, t12243: F, t24221: F, t1298: F, t5023: F, t81128: F, t81130: F, t81132: F, t81134: F, t81136: F, t81138: F, t24237: F, t689: F, t24245: F, t20292: F, t4186: F, t12305: F, t128: F, t22688: F, t43776: F, t606: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t81139, t81145, t81148, t81150, t81152) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3044::<F>(t25026, t3801, t1187, t1756, t58672, t69511, t1130, t24466, t1151, t58339, t6439, t12243, t24221);
        let t81153 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3045::<F>(t1298, t5023, t81128, t81130, t81132, t81134, t81136, t81138, t81139, t81145, t81148, t81150, t81152);
        let t81156 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3046::<F>(t24237, t689);
        let t81158 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3047::<F>(t24245, t689);
        let (t81160, t81162) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3048::<F>(t20292, t4186, t12305, t128);
        let (t81165, t81167) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3049::<F>(t22688, t43776, t606, t12305, t128);
    (t81145, t81148, t81150, t81152, t81153, t81156, t81158, t81160, t81162, t81165, t81167)
}

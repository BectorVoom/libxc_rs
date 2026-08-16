//! MGGA_C_REVTPSS lxc pol kernel — _part20_v4rho4_0 meta408 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1510;
use chunk1::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1511;
use chunk2::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1512;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_meta408<F: Float>(t11880: F, t3241: F, t1011: F, t1016: F, t2438: F, t3237: F, t697: F, t1010: F, t10345: F, t11883: F, t3244: F, t11766: F, t140: F, t1014: F, t11150: F, t1003: F, t11735: F, t221: F, t345: F, t346: F, t624: F, t1050: F, t41: F, t1012: F, t1017: F, t11767: F, t3236: F, t344: F, t348: F, t39443: F, t39449: F, sigma0: F, t1007: F, t11738: F, t3080: F, t3083: F, t1043: F, t11173: F, t11858: F, t16048: F, t11859: F, t11861: F, t11922: F, t11927: F, t11929: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t42712, t42716, t42719, t42721, t42724, t42727) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1510::<F>(t11880, t3241, t1011, t1016, t2438, t3237, t697, t1010, t10345, t11883, t3244, t11766, t140);
        let (t42748, t42752) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1511::<F>(t1014, t11150, t1003, t11735, t221, t345, t346, t624, t1050, t41, t1011, t1012, t1017, t11767, t3236, t3241, t344, t348, t39443, t39449, t42716, t42719, t42721, t42724, t42727, sigma0);
        let (t42754, t42756, t42760, t42765, t42769, t42772) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1512::<F>(t1007, t11738, t3080, t3083, t1043, t11173, t11858, t16048, t11859, t11861, t11922, t11927, t11929);
    (t42712, t42748, t42752, t42754, t42756, t42760, t42765, t42769, t42772)
}

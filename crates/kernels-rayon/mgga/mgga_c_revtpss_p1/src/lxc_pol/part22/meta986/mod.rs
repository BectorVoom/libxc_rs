//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta986 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3341;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3342;
use chunk2::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3343;
use chunk3::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3344;
use chunk4::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3345;
use chunk5::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3346;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta986(t11144: f64, t5825: f64, t2251: f64, t11142: f64, t128: f64, t63253: f64, t904: f64, t41281: f64, t41285: f64, t41287: f64, t41592: f64, t51937: f64, t51942: f64, t63266: f64, t63268: f64, t63274: f64, t63276: f64, t63278: f64, t63281: f64, t63285: f64, t18941: f64, t2258: f64, t41270: f64, t5819: f64, t18903: f64, t2850: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t63288, t63290) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3341(t11144, t5825, t2251, t11142, t128);
        let t63293 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3342(t128, t63253, t904);
        let t63295 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3343(t41281, t41285, t41287, t41592, t51937, t51942, t63266, t63268, t63274, t63276, t63278, t63281, t63285, t63290, t63293);
        let (t63297, t63299) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3344(t18941, t2258, t128, t904);
        let (t63302, t63304) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3345(t2251, t41270, t5819, t11142, t128);
        let (t63306, t63308) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3346(t18903, t2251, t128, t2850);
    (t63288, t63290, t63293, t63295, t63297, t63299, t63302, t63304, t63306, t63308)
}

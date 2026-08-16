//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta37 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk265;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk266;
use chunk2::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk267;
use chunk3::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk268;
use chunk4::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk269;
use chunk5::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk270;
use chunk6::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk271;
use chunk7::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk272;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta37(t235: f64, t240: f64, t234: f64, t243: f64, t807: f64, t236: f64, t786: f64, t27: f64, t124: f64, t800: f64, t213: f64, t225: f64, t232: f64, t239: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let t808 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk265(t235, t240);
        let t810 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk266(t234, t243, t808);
        let (t812, t813, t814) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk267(t807, t810, t236, t786, t240, t27);
        let (t815, t816) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk268(t243, t814, t124, t800);
        let (t819, t820) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk269(t815, t816, t813, t213, t225);
        let (t821, t822) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk270(t232);
        let t823 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk271(t235, t822);
        let t825 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk272(t239, t820, t823);
    (t808, t810, t812, t813, t814, t816, t819, t820, t821, t822, t823, t825)
}

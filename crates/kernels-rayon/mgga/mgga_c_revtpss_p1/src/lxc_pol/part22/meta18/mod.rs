//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta18 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk139;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk140;
use chunk2::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk141;
use chunk3::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk142;
use chunk4::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk143;
use chunk5::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk144;
use chunk6::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk145;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta18(t335: f64, t136: f64, t44: f64, t271: f64, t221: f64, t65: f64, t225: f64, t342: f64, t336: f64, t73: f64, t293: f64, t328: f64, t330: f64, sigma0: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t343, t344) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk139(t335, t136);
        let t345 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk140(t344, t44);
        let t346 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk141(t271);
        let (t348, t351) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk142(t221, t346, t65, t225, t342);
        let (t354, t355, t357) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk143(t336, t73, t225, t293, t328, t330);
        let (t358, t359) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk144(t357);
        let t360 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk145(sigma0);
    (t343, t344, t345, t346, t348, t351, t354, t355, t357, t358, t359, t360)
}

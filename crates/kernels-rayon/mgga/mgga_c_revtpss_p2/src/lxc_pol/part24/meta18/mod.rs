//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta18 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk142;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk143;
use chunk2::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk144;
use chunk3::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk145;
use chunk4::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk146;
use chunk5::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk147;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta18(t221: f64, t346: f64, t65: f64, t225: f64, t342: f64, t336: f64, t73: f64, t293: f64, t328: f64, t330: f64, sigma0: f64, t39: f64, t40: f64, rho0: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t348, t351, t354, t355, t357) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk142(t221, t346, t65, t225, t342, t336, t73, t293, t328, t330);
        let (t358, t359) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk143(t357);
        let t360 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk144(sigma0);
        let (t361, t365) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk145(t359, t360, t39, t40, rho0);
        let t366 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk146(t361, t365);
        let t367 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk147(t351, t366);
    (t348, t351, t354, t355, t357, t358, t359, t360, t361, t365, t366, t367)
}

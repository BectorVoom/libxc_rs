//! MGGA_C_REVTPSS lxc pol kernel — _part33_v4rho3sigma_8 meta19 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk135;
use chunk1::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk136;
use chunk2::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk137;
use chunk3::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk138;
use chunk4::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk139;
use chunk5::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk140;
use chunk6::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk141;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_meta19(t225: f64, t293: f64, t328: f64, t330: f64, t355: f64, sigma0: f64, t39: f64, t40: f64, rho0: f64, t351: f64, t335: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let t357 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk135(t225, t293, t328, t330, t355);
        let (t358, t359) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk136(t357);
        let t360 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk137(sigma0);
        let (t361, t362, t365) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk138(t359, t360, t39, t40, rho0);
        let t366 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk139(t361, t365);
        let (t367, t368) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk140(t351, t366, t335);
        let t369 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk141(t368);
    (t357, t358, t359, t360, t361, t362, t365, t366, t367, t368, t369)
}

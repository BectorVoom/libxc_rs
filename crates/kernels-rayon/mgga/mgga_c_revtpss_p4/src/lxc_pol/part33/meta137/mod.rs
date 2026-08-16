//! MGGA_C_REVTPSS lxc pol kernel — _part33_v4rho3sigma_8 meta137 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk737;
use chunk1::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk738;
use chunk2::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk739;
use chunk3::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk740;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_meta137(t3596: f64, t474: f64, t3147: f64, t479: f64, t3594: f64, t471: f64, t3153: f64, t1244: f64, t1121: f64, t414: f64, t66: f64, t3089: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t3598, t3599, t3600, t3603) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk737(t3596, t474, t3147, t479, t3594, t471);
        let (t3604, t3609, t3610, t3611, t3617) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk738(t3153, t3603, t1244, t3598, t3594, t471, t1121, t414);
        let (t3618, t3623) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk739(t3617, t66, t474, t479);
        let t3624 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk740(t3089, t3623);
    (t3598, t3599, t3600, t3603, t3604, t3609, t3610, t3611, t3617, t3618, t3623, t3624)
}

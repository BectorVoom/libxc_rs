//! MGGA_C_REVTPSS lxc pol kernel — _part30_v4rho3sigma_5 meta5 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk37;
use chunk1::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk38;
use chunk2::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk39;
use chunk3::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk40;
use chunk4::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk41;
use chunk5::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk42;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_meta5(t45: f64, t78: f64, t57: f64, t77: f64, t71: f64, t5: f64, t10: f64, t11: f64, t12: f64, t29: f64, t9: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t79, t80, t81) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk37(t45, t78, t57);
        let (t82, t83, t84) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk38(t57, t81, t80);
        let t85 = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk39(t77, t84);
        let (t88, t89, t90) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk40(t71, t85);
        let t91 = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk41(t90);
        let t93 = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk42(t5, t10, t11, t12, t29, t9, t91);
    (t79, t80, t81, t82, t83, t84, t85, t88, t89, t90, t91, t93)
}

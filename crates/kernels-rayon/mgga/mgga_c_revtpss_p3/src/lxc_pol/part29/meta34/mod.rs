//! MGGA_C_REVTPSS lxc pol kernel — _part29_v4rho3sigma_4 meta34 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk225;
use chunk1::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk226;
use chunk2::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk227;
use chunk3::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk228;
use chunk4::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk229;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_meta34(t112: f64, t625: f64, t111: f64, t43: f64, t605: f64, tau0: f64, t100: f64, t108: f64, t101: f64, t105: f64, t97: f64, t114: f64, t69: f64, t508: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t653, t654, t655) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk225(t112, t625, t111);
        let (t656, t658) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk226(t43, t605, tau0);
        let (t659, t661, t662, t665) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk227(t100, t658, t108, t101, t105, t656, t97);
        let (t666, t670) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk228(t114, t655, t665, t653, t69);
        let t671 = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk229(t508, t670);
    (t653, t654, t655, t656, t658, t659, t661, t662, t665, t666, t670, t671)
}

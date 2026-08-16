//! MGGA_C_REVTPSS lxc pol kernel — _part42_v4rho3tau_5 meta19 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk124;
use chunk1::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk125;
use chunk2::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk126;
use chunk3::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk127;
use chunk4::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk128;
use chunk5::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk129;
use chunk6::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk130;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_meta19(t357: f64, sigma0: f64, t39: f64, t40: f64, rho0: f64, t351: f64, t335: f64, t72: f64, t245: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t358, t359) = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk124(t357);
        let t360 = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk125(sigma0);
        let (t361, t362, t365) = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk126(t359, t360, t39, t40, rho0);
        let t366 = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk127(t361, t365);
        let (t367, t368) = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk128(t351, t366, t335);
        let t369 = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk129(t368);
        let (t370, t371) = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk130(t369, t72, t245);
    (t358, t359, t360, t361, t362, t365, t366, t367, t368, t369, t370, t371)
}

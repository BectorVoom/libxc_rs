//! MGGA_C_REVTPSS lxc pol kernel — _part41_v4rho3tau_4 meta178 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk742;
use chunk1::mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk743;
use chunk2::mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk744;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_meta178(t373: f64, t4772: f64, t371: f64, t372: f64, t225: f64, t4746: f64, t366: f64, t4589: f64, t4592: f64, t4594: f64, t4597: f64, t4634: f64, t4638: f64, t4716: f64, t4718: f64, t4721: f64, t4723: f64, t4727: f64, t4731: f64, t4736: f64) -> (f64, f64, f64, f64, f64) {
        let (t4852, t4854, t4857) = mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk742(t373, t4772, t371, t372, t225, t4746);
        let t4858 = mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk743(t366, t4857);
        let t4866 = mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk744(t4589, t4592, t4594, t4597, t4634, t4638, t4716, t4718, t4721, t4723, t4727, t4731, t4736);
    (t4852, t4854, t4857, t4858, t4866)
}

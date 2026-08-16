//! MGGA_C_REVTPSS lxc pol kernel — _part39_v4rho3tau_2 meta300 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1059;
use chunk1::mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1060;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_meta300(t2729: f64, t794: f64, t2732: f64, t136: f64, t860: f64, t2457: f64, t2710: f64, t10652: f64, t231: f64, t2783: f64, t2782: f64, t10069: f64, t2786: f64, t10073: f64, t836: f64, t251: f64, t2645: f64, t10111: f64, t22: f64, t870: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t10905, t10906, t10916, t10921, t10923) = mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1059(t2729, t794, t2732, t136, t860, t2457, t2710, t10652, t231, t2783, t2782, t10069, t2786);
        let (t10925, t10930, t10935, t10939) = mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1060(t10073, t2786, t231, t2783, t836, t860, t2782, t251, t2645, t10111, t22, t870);
    (t10905, t10906, t10916, t10921, t10923, t10925, t10930, t10935, t10939)
}

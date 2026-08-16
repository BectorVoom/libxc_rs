//! MGGA_C_REVTPSS lxc pol kernel — _part41_v4rho3tau_4 meta119 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk593;
use chunk1::mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk594;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_meta119(t300: f64, t960: f64, t2846: f64, t988: f64, t993: f64, t378: f64, t989: f64, t340: f64, t992: f64, t338: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
        let (t3022, t3037, t3046) = mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk593(t300, t960, t2846, t988, t993);
        let (t3047, t3052, t3056, t3057) = mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk594(t3046, t378, t989, t340, t992, t338);
    (t3022, t3037, t3046, t3047, t3052, t3056, t3057)
}

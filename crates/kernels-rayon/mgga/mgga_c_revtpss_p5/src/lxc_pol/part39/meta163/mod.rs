//! MGGA_C_REVTPSS lxc pol kernel — _part39_v4rho3tau_2 meta163 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk732;
use chunk1::mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk733;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_meta163(t3601: f64, t482: f64, t471: f64, t3153: f64, t1042: f64, t1244: f64, t3598: f64, t3594: f64, t1121: f64, t414: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t3602, t3603) = mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk732(t3601, t482, t471);
        let (t3604, t3605, t3606, t3609, t3610, t3611, t3612, t3613, t3617) = mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk733(t3153, t3603, t3602, t1042, t1244, t3598, t3594, t471, t1121, t414);
    (t3603, t3604, t3605, t3606, t3609, t3610, t3611, t3612, t3613, t3617)
}

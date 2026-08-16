//! MGGA_C_REVTPSS lxc pol kernel — _part41_v4rho3tau_4 meta92 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk525;
use chunk1::mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk526;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_meta92(t599: f64, t602: f64, t89: f64, t90: f64, t29: f64, t2: f64, t580: f64, t47: f64, t59: f64, t239: f64, t64: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
        let (t2242, t2246, t2247, t2255) = mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk525(t599, t602, t89, t90, t29, t2, t580);
        let (t2275, t2282, t2289) = mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk526(t47, t59, t239, t64);
    (t2242, t2246, t2247, t2255, t2275, t2282, t2289)
}

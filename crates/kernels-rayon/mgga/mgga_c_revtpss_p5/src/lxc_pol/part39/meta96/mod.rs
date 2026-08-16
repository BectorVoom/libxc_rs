//! MGGA_C_REVTPSS lxc pol kernel — _part39_v4rho3tau_2 meta96 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk536;
use chunk1::mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk537;
use chunk2::mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk538;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_meta96(t2258: f64, t36: f64, t70: f64, t607: f64, t627: f64, t362: f64, t41: f64, t47: f64, t2251: f64, t48: f64, t59: f64, t60: f64, sigma0: f64, t239: f64, t64: f64, t44: f64, t49: f64, t56: f64, t614: f64, t617: f64, t38: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t2259, t2260, t2263, t2270, t2275, t2276, t2279, t2282, t2283, t2286) = mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk536(t2258, t36, t70, t607, t627, t362, t41, t47, t2251, t48, t59, t60, sigma0);
        let t2289 = mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk537(t239, t64);
        let (t2290, t2291, t2292) = mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk538(t2289, t2270, t2276, t2279, t2283, t2286, t44, t49, t56, t614, t617, t38);
    (t2259, t2260, t2263, t2270, t2275, t2282, t2283, t2286, t2289, t2290, t2291, t2292)
}

//! MGGA_C_REVTPSS lxc pol kernel — _part20_v4rho4_0 meta74 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk474;
use chunk1::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk475;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_meta74(t47: f64, t2251: f64, t2258: f64, t48: f64, t59: f64, t60: f64, t239: f64, t64: f64, t2270: f64, t44: f64, t49: f64, t56: f64, t614: f64, t617: f64, t38: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
        let (t2275, t2276, t2279, t2282, t2283, t2286, t2289) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk474(t47, t2251, t2258, t48, t59, t60, t239, t64);
        let (t2291, t2292) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk475(t2289, t2270, t2276, t2279, t2283, t2286, t44, t49, t56, t614, t617, t38);
    (t2275, t2276, t2279, t2282, t2289, t2291, t2292)
}

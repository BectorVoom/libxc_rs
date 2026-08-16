//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta91 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk636;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk637;
use chunk2::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk638;
use chunk3::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk639;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta91(t30: f64, t33: f64, t2257: f64, zeta_threshold: f64, t36: f64, t70: f64, t607: f64, t627: f64, t362: f64, t41: f64, sigma0: f64, t47: f64, t2251: f64, t48: f64, t59: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let t2258 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk636(t30, t33, t2257, zeta_threshold);
        let (t2259, t2260, t2263, t2269, t2270) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk637(t2258, t36, t70, t607, t627, t362, t41, sigma0);
        let t2275 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk638(t47);
        let (t2276, t2279, t2282) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk639(t2251, t2275, t2258, t48, t59);
    (t2258, t2259, t2260, t2263, t2269, t2270, t2275, t2276, t2279, t2282)
}

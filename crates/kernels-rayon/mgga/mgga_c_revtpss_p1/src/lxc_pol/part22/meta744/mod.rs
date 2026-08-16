//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta744 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2812;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2813;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta744(t11007: f64, t252: f64, t786: f64, t11006: f64, t256: f64, t225: f64, t2441: f64, t39515: f64, t10504: f64, t138: f64, t886: f64, t9302: f64, t123: f64, t2465: f64, t9291: f64, t10982: f64, t860: f64, t9646: f64, t2434: f64, t2828: f64, t10115: f64, t251: f64, t887: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t41070, t41078, t41095, t41098) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2812(t11007, t252, t786, t11006, t256, t225, t2441, t39515, t10504, t138, t886, t9302);
        let (t41102, t41105, t41115, t41117, t41118) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2813(t123, t2465, t886, t9291, t10982, t860, t9646, t2434, t2828, t10115, t251, t887);
    (t41070, t41078, t41095, t41098, t41102, t41105, t41115, t41117, t41118)
}

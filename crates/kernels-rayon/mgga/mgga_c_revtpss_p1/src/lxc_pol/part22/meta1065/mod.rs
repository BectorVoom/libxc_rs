//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta1065 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3814;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3815;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta1065(t5591: f64, t2608: f64, t512: f64, t6800: f64, t177: f64, t21931: f64, t762: f64, t48222: f64, t48225: f64, t48227: f64, t48230: f64, t46973: f64, t198: f64, t3828: f64, t39483: f64, t39520: f64, t39528: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t73345, t73350, t73353, t73354, t73355, t73356, t73357, t73358) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3814(t5591, t2608, t512, t6800, t177, t21931, t762, t48222, t48225, t48227, t48230, t46973);
        let t73359 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3815(t198, t3828, t39483, t39520, t39528, t73345, t73350, t73353, t73354, t73355, t73356, t73357, t73358);
    (t73345, t73350, t73353, t73354, t73355, t73356, t73357, t73358, t73359)
}

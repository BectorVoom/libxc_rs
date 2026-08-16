//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3814/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3814(t5591: f64, t2608: f64, t512: f64, t6800: f64, t177: f64, t21931: f64, t762: f64, t48222: f64, t48225: f64, t48227: f64, t48230: f64, t46973: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t73345 = t5591 * t5591;
    let t73350 = t512 * t6800 * t2608;
    let t73352 = t21931 * t177 * t762;
    let t73353 = 0.11696447245269292414e1_f64 * t73352;
    let t73354 = 0.11696447245269292414e1_f64 * t48222;
    let t73355 = 8.0_f64 * t48225;
    let t73356 = 120.0_f64 * t48227;
    let t73357 = 2.0_f64 * t48230;
    let t73358 = 24.0_f64 * t46973;
    (t73345, t73350, t73353, t73354, t73355, t73356, t73357, t73358)
}

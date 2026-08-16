//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1549/1850 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1549(t20816: f64, t5293: f64, t24611: f64, t3172: f64, t3711: f64, t24252: f64, t300: f64, t17529: f64, t20786: f64, t21102: f64, t5265: f64, t5274: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t82338 = t5293 * t20816;
    let t82351 = t3711 * t3172 * t24611;
    let t82389 = t300 * t24252;
    let t82434 = t17529 * t20786;
    let t82441 = t21102 * t5265;
    let t82457 = t5274 * t20816;
    (t82338, t82351, t82389, t82434, t82441, t82457)
}

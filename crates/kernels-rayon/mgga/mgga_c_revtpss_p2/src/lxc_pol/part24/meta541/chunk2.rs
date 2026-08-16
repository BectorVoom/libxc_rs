//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1591/1850 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1591(t21820: f64, t22628: f64, t2339: f64, t4263: f64, t46143: f64, t46157: f64, t49698: f64, t5915: f64, t655: f64, t69: f64, t75540: f64, t75639: f64, t75822: f64, t75831: f64, t75843: f64, t86981: f64, t86988: f64, t87046: f64) -> f64 {
    let t87050 = t46143 + 616.0_f64 / 27.0_f64 * t49698 + 44.0_f64 / 3.0_f64 * t75639 - 22.0_f64 / 3.0_f64 * t75540 + 8.0_f64 * t75822 - 8.0_f64 * t75831 + 4.0_f64 / 3.0_f64 * t75843 + 3.0_f64 * t69 * t46157 * t86981 - 9.0_f64 / 2.0_f64 * t69 * t21820 * t5915 + 3.0_f64 / 4.0_f64 * t69 * t2339 * t86988 + t69 * t4263 * t22628 - t69 * t655 * t87046 / 8.0_f64;
    t87050
}

//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2803/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2803(t13458: f64, t21820: f64, t21876: f64, t22589: f64, t22628: f64, t2339: f64, t31035: f64, t4263: f64, t4287: f64, t46157: f64, t5915: f64, t655: f64, t665: f64, t69: f64, t75542: f64, t75822: f64, t75831: f64, t75833: f64, t75843: f64, t75887: f64, t75924: f64) -> f64 {
    let t75929 = t75542 + 2.0_f64 * t75822 + 3.0_f64 * t69 * t46157 * t22589 * t665 - 9.0_f64 / 4.0_f64 * t69 * t21820 * t4287 - 2.0_f64 * t75831 - 9.0_f64 / 4.0_f64 * t31035 * t75833 * t665 + 3.0_f64 / 4.0_f64 * t69 * t13458 * t5915 + 3.0_f64 / 4.0_f64 * t69 * t4263 * t21876 + t75843 / 3.0_f64 + t69 * t2339 * t22628 * t665 / 4.0_f64 - t69 * t655 * (t75887 + t75924) / 8.0_f64;
    t75929
}

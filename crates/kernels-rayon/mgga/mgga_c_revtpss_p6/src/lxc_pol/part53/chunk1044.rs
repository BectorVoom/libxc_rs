//! MGGA_C_REVTPSS lxc pol — lxc_pol part 53 (v4rho2sigma2_8) CSE chunk 1044/1244 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part53_v4rho2sigma2_8_chunk1044(t32299: f64, t32301: f64, t32303: f64, t32305: f64, t32307: f64, t32309: f64, t32312: f64, t32320: f64, t32323: f64, t32325: f64, t32329: f64, t32338: f64, t32340: f64, t6985: f64, t7591: f64) -> f64 {
    let t32883 = -2.0_f64 * t6985 * t7591 + t32299 - 2.0_f64 * t32301 - 2.0_f64 * t32303 - 2.0_f64 * t32305 - 2.0_f64 * t32307 - 2.0_f64 * t32309 - 2.0_f64 * t32312 - t32320 + t32323 - 2.0_f64 * t32325 + t32329 - t32338 - t32340;
    t32883
}

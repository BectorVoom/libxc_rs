//! MGGA_C_REVTPSS lxc pol — lxc_pol part 53 (v4rho2sigma2_8) CSE chunk 1181/1244 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part53_v4rho2sigma2_8_chunk1181(t126125: f64, t32474: f64, t4469: f64, t8477: f64, t31844: f64, t826: f64, t126046: f64, t247: f64, t31752: f64, t4366: f64, t126250: f64, t31805: f64) -> (f64, f64, f64, f64) {
    let t126271 = t32474 * t126125;
    let t126273 = t8477 * t4469;
    let t126276 = t31844 * t826;
    let t126280 = t31752 * t126276 * t247 * t126046 * t4366;
    let t126282 = t31805 * t126250;
    (t126271, t126273, t126280, t126282)
}

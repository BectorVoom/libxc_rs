//! MGGA_C_REVTPSS lxc pol — lxc_pol part 53 (v4rho2sigma2_8) CSE chunk 106/1244 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part53_v4rho2sigma2_8_chunk106(t344: f64, t44: f64, t271: f64, t221: f64, t65: f64, t225: f64, t342: f64) -> (f64, f64, f64, f64) {
    let t345 = t44 * t344;
    let t346 = 1.0_f64 / t271;
    let t348 = t221 * t65 * t346;
    let t351 = t342 * t225;
    (t345, t346, t348, t351)
}

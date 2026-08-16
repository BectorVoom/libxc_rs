//! MGGA_C_REVTPSS lxc pol — lxc_pol part 52 (v4rho2sigma2_7) CSE chunk 973/1292 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part52_v4rho2sigma2_7_chunk973(t72: f64, t8015: f64, t686: f64, t7058: f64, t7064: f64, t689: f64, t8011: f64, t25431: f64, t25411: f64, t786: f64, t7998: f64, t789: f64) -> (f64, f64, f64, f64, f64) {
    let t28359 = t8015 * t72;
    let t28360 = t28359 * t686;
    let t28361 = t7058 * t28360;
    let t28366 = t7064 * t28360;
    let t28368 = t8011 * t689;
    let t28369 = t25431 * t28368;
    let t28371 = t25411 * t28368;
    let t28373 = t786 * t7998;
    let t28374 = t28373 * t789;
    (t28361, t28366, t28369, t28371, t28374)
}

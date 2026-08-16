//! MGGA_C_REVTPSS lxc pol — lxc_pol part 53 (v4rho2sigma2_8) CSE chunk 831/1244 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part53_v4rho2sigma2_8_chunk831(t14685: f64, t220: f64, t125: f64, t4343: f64, t221: f64, t4433: f64, t1501: f64, t670: f64) -> (f64, f64, f64, f64) {
    let t14686 = t14685 * t220;
    let t14691 = t125 * t4343;
    let t14756 = t221 * t4433;
    let t18227 = t1501 * t670;
    (t14686, t14691, t14756, t18227)
}

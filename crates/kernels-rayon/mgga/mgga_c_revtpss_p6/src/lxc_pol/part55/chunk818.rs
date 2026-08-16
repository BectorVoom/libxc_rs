//! MGGA_C_REVTPSS lxc pol — lxc_pol part 55 (v4rho2sigma2_10) CSE chunk 818/1306 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part55_v4rho2sigma2_10_chunk818(t532: f64, t8713: f64, t1450: f64, t2014: f64, t2033: f64, t4147: f64) -> (f64, f64, f64, f64) {
    let t8714 = t532 * t8713;
    let t8715 = t8714 * t1450;
    let t8716 = t2014 * t8715;
    let t8717 = t4147 * t2033;
    (t8714, t8715, t8716, t8717)
}

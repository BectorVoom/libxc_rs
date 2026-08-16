//! MGGA_C_REVTPSS lxc pol — lxc_pol part 31 (v4rho3sigma_6) CSE chunk 2188/2259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2188(t30123: f64, t95088: f64, t670: f64, t7724: f64, t1353: f64, t6922: f64, t25082: f64, t8717: f64, t30088: f64, t689: f64, t25904: f64, t25899: f64) -> (f64, f64, f64, f64, f64) {
    let t108117 = 6.0_f64 * t95088 * t30123;
    let t108120 = t7724 * t670;
    let t108126 = t6922 * t1353;
    let t108129 = 3.0_f64 * t25082 * t8717 * t108126;
    let t108132 = t30088 * t689;
    let t108133 = t25904 * t108132;
    let t108135 = t25899 * t108132;
    (t108117, t108120, t108129, t108133, t108135)
}

//! MGGA_C_REVTPSS lxc pol — lxc_pol part 28 (v4rho3sigma_3) CSE chunk 2139/2277 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2139(t13716: f64, t1450: f64, t2014: f64, t7237: f64, t18163: f64, t7735: f64, t27137: f64, t4254: f64, t25082: f64, t75353: f64, t8717: f64, t7311: f64, t9593: f64) -> (f64, f64, f64, f64, f64) {
    let t98564 = t1450 * t13716;
    let t98567 = 3.0_f64 * t2014 * t7237 * t98564;
    let t98569 = 2.0_f64 * t18163 * t7735;
    let t98571 = 4.0_f64 * t4254 * t27137;
    let t98574 = 6.0_f64 * t25082 * t8717 * t75353;
    let t98575 = t7311 * t9593;
    (t98567, t98569, t98571, t98574, t98575)
}

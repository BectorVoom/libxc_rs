//! MGGA_C_REVTPSS lxc pol — lxc_pol part 33 (v4rho3sigma_8) CSE chunk 1407/2275 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1407(t14671: f64, t14686: f64, t4366: f64, t14931: f64, t136: f64, t1568: f64, t2457: f64, t2710: f64, t2470: f64, t4522: f64, t874: f64, t4469: f64, t822: f64) -> (f64, f64, f64, f64, f64) {
    let t14933 = t14686 * t14671 * t4366;
    let t14934 = t14931 * t14933;
    let t14946 = t1568 * t136;
    let t14948 = t2710 * t14946 * t2457;
    let t14951 = t874 * t4522 * t2470;
    let t14972 = t822 * t4469;
    (t14933, t14934, t14948, t14951, t14972)
}

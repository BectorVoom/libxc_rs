//! MGGA_C_REVTPSS lxc pol — lxc_pol part 52 (v4rho2sigma2_7) CSE chunk 830/1292 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part52_v4rho2sigma2_7_chunk830(t3335: f64, t389: f64, t1077: f64, t992: f64, t1031: f64, t4171: f64, t602: f64, t1466: f64, t2246: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t11108 = 1.0_f64 / t3335 / t389;
    let t11119 = t1077 * t1077;
    let t11120 = 1.0_f64 / t11119;
    let t11198 = t992 * t992;
    let t11199 = 1.0_f64 / t11198;
    let t11238 = t1031 * t1031;
    let t11239 = 1.0_f64 / t11238;
    let t13269 = t4171 * t602;
    let t13272 = t1466 * t2246;
    (t11108, t11120, t11199, t11239, t13269, t13272)
}

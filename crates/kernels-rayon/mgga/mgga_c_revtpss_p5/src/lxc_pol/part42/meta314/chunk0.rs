//! MGGA_C_REVTPSS lxc pol — lxc_pol part 42 (v4rho3tau_5) CSE chunk 1086/1505 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1086(t10290: f64, t4171: f64, t602: f64, t1466: f64, t2246: f64, t580: f64, t9342: f64, t116: f64, t4245: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t13266 = 252.0_f64 * t10290;
    let t13269 = t4171 * t602;
    let t13272 = t1466 * t2246;
    let t13309 = 2.0_f64 * t580;
    let t13310 = 6.0_f64 * t9342;
    let t13426 = t4245 * t116;
    (t13266, t13269, t13272, t13309, t13310, t13426)
}

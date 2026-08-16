//! MGGA_C_REVTPSS lxc pol — lxc_pol part 35 (v4rho3sigma_10) CSE chunk 1057/1234 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part35_v4rho3sigma_10_chunk1057(t2062: f64, t867: f64, t786: f64, t2470: f64, t7406: f64, t7064: f64, t136: f64, t2066: f64, t2457: f64, t25299: f64, t25305: f64, t7058: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t26496 = t2062 * t867;
    let t26497 = t786 * t26496;
    let t26506 = t7406 * t2470;
    let t26508 = 0.17135234354032049604e-1_f64 * t7064 * t26506;
    let t26518 = t2066 * t136;
    let t26519 = t26518 * t2457;
    let t26521 = 0.17135234354032049604e-2_f64 * t25299 * t26519;
    let t26534 = 0.22849835011101738147e-2_f64 * t25305 * t26519;
    let t26536 = 0.96373646535613327357e-2_f64 * t7058 * t26506;
    (t26496, t26497, t26506, t26508, t26518, t26519, t26521, t26534, t26536)
}

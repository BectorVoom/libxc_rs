//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 132/1383 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk132(t367: f64, t373: f64, t198: f64, t287: f64, t322: f64, t324: f64, t330: f64, t259: f64) -> (f64, f64, f64) {
    let t375 = t367 * t373 + 1.0_f64;
    let t376 = f64::ln(t375);
    let t379 = t198 * t330 * t376 - t287 + t322 + t324;
    let t380 = t259 < t379;
    let t381 = piecewise3(t380, t379, t259);
    (t375, t381, t379)
}

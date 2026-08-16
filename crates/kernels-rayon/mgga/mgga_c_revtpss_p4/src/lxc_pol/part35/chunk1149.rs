//! MGGA_C_REVTPSS lxc pol — lxc_pol part 35 (v4rho3sigma_10) CSE chunk 1149/1234 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part35_v4rho3sigma_10_chunk1149(t2453: f64, t2458: f64, t7998: f64, t14485: f64, t26497: f64, t10073: f64, t25402: f64, t7056: f64, t7997: f64, t26519: f64, t98867: f64, t136: f64, t2457: f64, t8015: f64) -> (f64, f64, f64, f64, f64) {
    let t103161 = t2453 * t7998 * t2458;
    let t103220 = t26497 * t14485;
    let t103234 = t10073 * t7056 * t25402 * t7997;
    let t103240 = t98867 * t26519;
    let t103363 = t8015 * t136 * t2457;
    (t103161, t103220, t103234, t103240, t103363)
}

//! MGGA_C_REVTPSS lxc pol — lxc_pol part 25 (v4rho3sigma_0) CSE chunk 474/1360 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part25_v4rho3sigma_0_chunk474(t2516: f64, t760: f64, t675: f64, t681: f64, t268: f64, t702: f64) -> (f64, f64, f64) {
    let t2518 = 0.5848223622634646207e0_f64 * t760 * t2516;
    let t2519 = t675 * t681;
    let t2522 = 0.35616666666666666666e-1_f64 * t268 * t2519 * t702;
    (t2518, t2519, t2522)
}

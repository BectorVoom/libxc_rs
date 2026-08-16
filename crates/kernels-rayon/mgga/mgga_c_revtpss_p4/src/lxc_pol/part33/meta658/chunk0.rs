//! MGGA_C_REVTPSS lxc pol — lxc_pol part 33 (v4rho3sigma_8) CSE chunk 2116/2275 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2116(t29643: f64, t686: f64, t72: f64, t93281: f64, t93317: f64, t18451: f64, t25270: f64, t18462: f64, t18647: f64, t18527: f64, t98988: f64, t18471: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t105973 = t29643 * t72 * t686;
    let t105974 = t93281 * t105973;
    let t105976 = t93317 * t105973;
    let t105985 = t25270 * t18451;
    let t105987 = t25270 * t18462;
    let t105989 = t25270 * t18647;
    let t105991 = t98988 * t18527;
    let t105993 = t25270 * t18471;
    (t105974, t105976, t105985, t105987, t105989, t105991, t105993)
}

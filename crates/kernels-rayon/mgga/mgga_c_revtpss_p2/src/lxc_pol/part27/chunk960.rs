//! MGGA_C_REVTPSS lxc pol — lxc_pol part 27 (v4rho3sigma_2) CSE chunk 960/1333 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part27_v4rho3sigma_2_chunk960(t3022: f64, t3026: f64, t11467: f64, t3011: f64, t973: f64, t981: f64, t2986: f64, t972: f64, t3007: f64, t11465: f64, t3014: f64, t11501: f64, t964: f64) -> (f64, f64, f64, f64, f64) {
    let t11596 = 0.35089341735807877242e1_f64 * t3022 * t3026;
    let t11598 = t3011 * t11467 * t973;
    let t11600 = 0.35089341735807877242e1_f64 * t981 * t11598;
    let t11601 = t2986 * t972;
    let t11602 = t11601 * t3007;
    let t11604 = 0.35089341735807877242e1_f64 * t981 * t11602;
    let t11606 = t11465 * t11467 * t3014;
    let t11608 = 0.10389515463408878255e3_f64 * t981 * t11606;
    let t11610 = t964 * t11501 * t973;
    (t11596, t11600, t11604, t11608, t11610)
}

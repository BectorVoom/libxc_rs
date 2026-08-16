//! MGGA_C_REVTPSS lxc pol — lxc_pol part 30 (v4rho3sigma_5) CSE chunk 2126/2270 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2126(t7234: f64, t8995: f64, t28199: f64, t28021: f64, t7235: f64, t13648: f64, t2014: f64, t7312: f64, t25861: f64, t7732: f64, t2322: f64, t28056: f64) -> (f64, f64, f64, f64, f64) {
    let t98588 = t7234 * t8995;
    let t98590 = 4.0_f64 * t98588 * t28199;
    let t98594 = 2.0_f64 * t7235 * t28021;
    let t98597 = 2.0_f64 * t2014 * t7312 * t13648;
    let t98599 = 4.0_f64 * t7732 * t25861;
    let t98601 = 4.0_f64 * t2322 * t28056;
    (t98590, t98594, t98597, t98599, t98601)
}

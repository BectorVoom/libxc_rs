//! MGGA_C_REVTPSS lxc pol — lxc_pol part 52 (v4rho2sigma2_7) CSE chunk 173/1292 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part52_v4rho2sigma2_7_chunk173(t128: f64, t72: f64, t686: f64, t3: f64, t66: f64, t124: f64, t138: f64) -> (f64, f64, f64, f64) {
    let t691 = f64::sqrt(t128);
    let t692 = t691 * t72;
    let t693 = t692 * t686;
    let t696 = 1.0_f64 / t66 / t3;
    let t697 = t124 * t696;
    let t698 = t138 * t697;
    (t692, t693, t696, t698)
}

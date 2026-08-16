//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2498/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2498(t49876: f64, t22: f64, t39454: f64, t4398: f64, t9387: f64, t14362: f64, t9575: f64, t123: f64, t2630: f64, t4392: f64, t9318: f64, t14322: f64, t2516: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t49877 = 36.0_f64 * t49876;
    let t49886 = 12.0_f64 * t22;
    let t49887 = 24.0_f64 * t39454;
    let t49897 = t4398 * t9387;
    let t49926 = t14362 * t9575;
    let t49929 = t4392 * t123 * t2630;
    let t49930 = 0.32530743900905219526e-1_f64 * t49929;
    let t49940 = t4398 * t9318;
    let t49957 = t14322 * t2516;
    (t49877, t49886, t49887, t49897, t49926, t49930, t49940, t49957)
}

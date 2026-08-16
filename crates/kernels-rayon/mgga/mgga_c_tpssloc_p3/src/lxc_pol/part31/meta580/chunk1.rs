//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 1819/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1819(t81284: f64, t26203: f64, t6883: f64, t6897: f64, t7700: f64, t80645: f64, t214: f64, t5318: f64, t81311: f64, t26378: f64, t6914: f64, t1372: f64, t1799: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t90706 = 0.3289868133696452873e-1_f64 * t81284;
    let t90707 = t6883 * t26203;
    let t90723 = t6897 * t80645 * t7700;
    let t90739 = t214 * t5318;
    let t90743 = 0.16449340668482264365e-1_f64 * t81311;
    let t90749 = t6914 * t26378;
    let t90754 = t1372 * t1799;
    (t90706, t90707, t90723, t90739, t90743, t90749, t90754)
}

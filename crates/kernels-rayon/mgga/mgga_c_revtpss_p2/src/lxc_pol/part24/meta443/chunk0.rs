//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1401/1850 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1401(t39494: f64, t3964: f64, t4096: f64, t2453: f64, t9679: f64, t3906: f64, t3907: f64, t1359: f64, t39501: f64, t10115: f64, t555: f64, t123: f64, t125: f64, t1358: f64, t8779: f64, t9645: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t47454 = 0.20561456923286030469e-1_f64 * t3964 * t4096 * t39494;
    let t47480 = t2453 * t9679;
    let t47504 = 0.20561456923286030469e-1_f64 * t3906 * t3907 * t39494;
    let t47561 = 0.56911289235245161963e-1_f64 * t39501 * t1359;
    let t47567 = t10115 * t555;
    let t47591 = 0.65457331274007190912e-5_f64 * t123 * t125 * t8779 * t9645 * t555 * t1358;
    (t47454, t47480, t47504, t47561, t47567, t47591)
}

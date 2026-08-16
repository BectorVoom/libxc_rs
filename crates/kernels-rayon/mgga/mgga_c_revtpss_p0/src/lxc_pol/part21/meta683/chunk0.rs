//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2497/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2497(t3718: f64, t3722: f64, t44546: f64, t3566: f64, t3766: f64, t5330: f64, t12646: f64, t12915: f64, t247: f64, t5384: f64, t12831: f64, t12865: f64) -> (f64, f64, f64, f64) {
    let t44548 = t3718 * t44546 * t3722;
    let t44550 = t3566 * t3766;
    let t44551 = t44550 * t5330;
    let t44559 = t5384 * t247 * t12915 * t12646;
    let t44561 = t12831 * t12865;
    (t44548, t44551, t44559, t44561)
}

//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1248/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1248(t104: f64, t108: f64, t13056: f64, t176: f64, t185: f64, t1879: f64, t20063: f64, t203: f64, t22434: f64, t22439: f64, t22657: f64, t22659: f64, t22661: f64, t38936: f64, t55933: f64, t56062: f64, t56068: f64, t56638: f64, t714: f64, t95: f64) -> f64 {
    let t56643 = t22434 - t22439 - t22657 + t56062 - t22659 - t22661 + t176 * t185 * t55933 * t108 * t203 / 2.0_f64 - t56068 + 140.0_f64 / 3.0_f64 * t38936 - 0.93041573165652349787e-1_f64 * t1879 * t13056 * t20063 + 0.25844881434903430496e-2_f64 * t95 * t104 * t56638 * t714;
    t56643
}

//! MGGA_C_KCIS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 523/1419 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part5_v3rho3_2_chunk523(t2491: f64, t62: f64, t2490: f64, t752: f64, t128: f64, t88: f64, t109: f64, t15: f64, t113: f64, t143: f64, t130: f64, t647: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t2492 = t62 * t2491;
    let t2493 = t2490 * t2492;
    let t2494 = t752 * t2493;
    let t2496 = t88 * t128;
    let t2500 = t109 * t15;
    let t2507 = t143 * t113;
    let t2508 = t647 * t130;
    (t2492, t2493, t2494, t2496, t2500, t2507, t2508)
}

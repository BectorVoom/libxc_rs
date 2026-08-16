//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 48 (v4rho2sigma2_4) CSE chunk 1000/1034 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part48_v4rho2sigma2_4_chunk1000(t114264: f64, t114270: f64, t114279: f64, t114288: f64, t114292: f64, t115596: f64, t115601: f64, t115617: f64, t115619: f64, t12033: f64, t1375: f64, t1386: f64, t22670: f64, t24088: f64, t24092: f64, t24095: f64, t31564: f64, t3752: f64, t3882: f64, t3887: f64, t3911: f64, t568: f64, t6958: f64, t6963: f64, t7199: f64, t8617: f64, t8636: f64, t8637: f64) -> f64 {
    let t115622 = t3752 * t8617 * t568 + 4.0_f64 * t3882 * t31564 - 0.76763589786250567036e-1_f64 * t115596 + t114264 + t114270 - t114279 + 4.0_f64 * t24095 * t6963 + t114288 + 0.82246703342411321824e-2_f64 * t115601 + 2.0_f64 * t6958 * t24088 + 2.0_f64 * t1375 * t3887 * t8636 * t3911 + 4.0_f64 * t22670 * t7199 - t12033 * t8637 - 6.0_f64 * t6958 * t24092 + t114292 + 0.3289868133696452873e-1_f64 * t115617 - 2.0_f64 * t115619 * t1386;
    t115622
}

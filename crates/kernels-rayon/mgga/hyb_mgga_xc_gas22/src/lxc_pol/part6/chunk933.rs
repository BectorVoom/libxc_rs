//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 933/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk933(t136: f64, t8453: f64, t191: f64, t686: f64, t214: f64, t3157: f64, t677: f64, t2002: f64, t3160: f64, t2028: f64, t1240: f64, t2024: f64, t2027: f64, t2154: f64, t2949: f64, t2986: f64, t3124: f64, t684: f64, t687: f64, t7831: f64, t8224: f64, t8441: f64, t8446: f64, t8450: f64, t8452: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t8454 = t136 * t8453;
    let t8456 = t686 * t191;
    let t8457 = t8456 * t214;
    let t8462 = t677 * t3157 / 32.0_f64;
    let t8465 = t3160 * t2002;
    let t8469 = t3160 * t2028;
    let t8473 = t8224 / 96.0_f64 - 3.0_f64 / 64.0_f64 * t136 * t8441 - 3.0_f64 / 64.0_f64 * t1240 * t2154 + 3.0_f64 / 16.0_f64 * t2949 * t8446 - t8450 - t8452 + t8454 / 96.0_f64 + t684 * t2986 * t8457 / 32.0_f64 - t8462 + 3.0_f64 / 32.0_f64 * t7831 * t3124 - t684 * t687 * t8465 / 64.0_f64 - t2024 * t2027 * t8469 / 48.0_f64;
    (t8454, t8456, t8457, t8462, t8465, t8469, t8473)
}

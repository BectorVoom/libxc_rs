//! MGGA_C_KCIS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 929/1419 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part5_v3rho3_2_chunk929(t122: f64, t160: f64, t2379: f64, t2434: f64, t2439: f64, t2440: f64, t2459: f64, t2477: f64, t60: f64, t684: f64, t718: f64, t728: f64, t745: f64, t8556: f64, t8561: f64, t8604: f64, t8747: f64, t8969: f64, t8972: f64, t8979: f64, t8996: f64, t97: f64) -> f64 {
    let t8998 = -0.70279601891642686494e-2_f64 * t160 * t97 - 0.14055920378328537299e-1_f64 * t8969 * t728 - 0.21083880567492805948e-1_f64 * t8972 * t2440 + 0.70279601891642686494e-2_f64 * t2434 * t2459 - 0.28111840756657074598e-1_f64 * t8979 * t8556 + 0.21083880567492805948e-1_f64 * t2439 * t8561 - 0.23426533963880895498e-2_f64 * t718 * t8604 - t8747 * t122 - 3.0_f64 * t2379 * t745 - 3.0_f64 * t684 * t2477 - t60 * t8996;
    t8998
}

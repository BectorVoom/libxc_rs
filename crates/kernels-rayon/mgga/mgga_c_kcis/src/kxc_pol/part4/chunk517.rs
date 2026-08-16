//! MGGA_C_KCIS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 517/1420 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_kxc_pol_part4_v3rho3_1_chunk517(t160: f64, t2466: f64, t2471: f64, t2475: f64, t740: f64, t122: f64, t2379: f64, t2434: f64, t2439: f64, t2440: f64, t2459: f64, t60: f64, t684: f64, t718: f64, t728: f64, t745: f64, t97: f64) -> (f64, f64) {
    let t2477 = -0.11955719325063177623e-1_f64 * t740 + 0.40985e-2_f64 * t2466 - 0.10566666666666666667e-2_f64 * t2471 + 0.3884654180847230157e-4_f64 * t160 - 0.420109375e-5_f64 * t2475;
    let t2479 = 0.23426533963880895498e-2_f64 * t740 * t97 + 0.46853067927761790996e-2_f64 * t2434 * t728 + 0.70279601891642686494e-2_f64 * t2439 * t2440 - 0.23426533963880895498e-2_f64 * t718 * t2459 - t2379 * t122 - 2.0_f64 * t684 * t745 - t60 * t2477;
    (t2477, t2479)
}

//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 879/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk879(t1096: f64, t7410: f64, t2783: f64, t2791: f64, t221: f64, t454: f64, t7345: f64, t7337: f64, t7340: f64, t7343: f64, t7346: f64, t7350: f64, t7352: f64, t7354: f64, t7357: f64) -> (f64, f64, f64, f64) {
    let t7411 = t7410 * t1096;
    let t7420 = t2783 * t2791;
    let t7426 = 0.34450798614814814813e-2_f64 * t221 * t7345 * t454;
    let t7435 = -0.25319e1_f64 * t7337 + 0.16879333333333333333e1_f64 * t7340 - 0.19692555555555555555e1_f64 * t7343 - 0.93011851851851851854e0_f64 * t7346 + 0.13651666666666666667e0_f64 * t7350 - 0.27303333333333333333e0_f64 * t7352 - 0.3185388888888888889e0_f64 * t7354 - 0.36514074074074074075e0_f64 * t7357;
    (t7411, t7420, t7426, t7435)
}

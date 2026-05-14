//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 857/1345 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk857<F: Float>(t1096: F, t7410: F, t2783: F, t2791: F, t221: F, t454: F, t7345: F, t7337: F, t7340: F, t7343: F, t7346: F, t7350: F, t7352: F, t7354: F, t7357: F, t1047: F) -> (F, F, F, F, F) {
    let t7411 = t7410 * t1096;
    let t7420 = t2783 * t2791;
    let t7426 = 0.34450798614814814813e-2 * t221 * t7345 * t454;
    let t7435 = -0.25319e1 * t7337 + 0.16879333333333333333e1 * t7340 - 0.19692555555555555555e1 * t7343 - 0.93011851851851851854e0 * t7346 + 0.13651666666666666667e0 * t7350 - 0.27303333333333333333e0 * t7352 - 0.3185388888888888889e0 * t7354 - 0.36514074074074074075e0 * t7357;
    let t7436 = t7435 * t1047;
    (t7411, t7420, t7426, t7435, t7436)
}

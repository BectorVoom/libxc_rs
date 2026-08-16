//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 878/1455 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk878<F: Float>(t483: F, t7253: F, t2639: F, t7238: F, t7337: F, t7340: F, t7343: F, t7346: F, t7350: F, t7352: F, t7354: F, t7357: F) -> (F, F, F) {
    let t7398 = t483 * t7253;
    let t7399 = t7238 * t2639;
    let t7410 = -F::cast_from(0.34523333333333333333e1_f64) * t7337 + F::cast_from(0.23015555555555555556e1_f64) * t7340 - F::cast_from(0.26851481481481481482e1_f64) * t7343 - F::cast_from(0.93932222222222222223e0_f64) * t7346 + F::cast_from(0.73355e-1_f64) * t7350 - F::cast_from(0.14671e0_f64) * t7352 - F::cast_from(0.17116166666666666667e0_f64) * t7354 - F::cast_from(0.36793333333333333333e0_f64) * t7357;
    (t7398, t7399, t7410)
}

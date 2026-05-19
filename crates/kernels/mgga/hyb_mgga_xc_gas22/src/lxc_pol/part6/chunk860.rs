//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 860/1455 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk860<F: Float>(t348: F, t355: F, t6966: F, t345: F, t238: F, t353: F, t6611: F, t2213: F, t963: F) -> (F, F, F, F, F, F, F) {
    let t7009 = F::new(1.0) / t348 / t355 / F::new(4.0);
    let t7016 = F::new(28.0) / F::new(27.0) * t6966;
    let t7021 = F::cast_from(0.16068111111111111111e1_f64) * t6966;
    let t7025 = F::new(1.0)/pow_3_2::<F>(t345);
    let t7034 = t238 * t6611 * t353;
    let t7035 = F::cast_from(0.46308888888888888888e0_f64) * t7034;
    let t7037 = t238 * t2213 * t963;
    (t7009, t7016, t7021, t7025, t7034, t7035, t7037)
}

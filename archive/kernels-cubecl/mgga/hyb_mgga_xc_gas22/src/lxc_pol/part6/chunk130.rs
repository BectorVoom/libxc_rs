//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 130/1455 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk130<F: Float>(t238: F, t242: F, t353: F, t345: F, t348: F, t351: F) -> (F, F, F, F) {
    let t355 = t238 * t242 * t353;
    let t357 = F::cast_from(0.379785e1_f64) * t348 + F::cast_from(0.8969e0_f64) * t345 + F::cast_from(0.204775e0_f64) * t351 + F::cast_from(0.123235e0_f64) * t355;
    let t360 = F::cast_from(1.0_f64) + F::cast_from(0.16081979498692535067e2_f64) / t357;
    let t361 = F::ln(t360);
    (t355, t357, t360, t361)
}

//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 531/1455 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk531<F: Float>(t2479: F, t2523: F, t2521: F, t2454: F, t2457: F, t2468: F, t974: F, t978: F) -> (F, F, F, F, F) {
    let t2524 = t2479 * t2523;
    let t2526 = F::cast_from(0.16081979498692535067e2_f64) * t2521 * t2524;
    let t2527 = F::cast_from(0.22831111111111111111e-1_f64) * t2454;
    let t2530 = t2527 - F::cast_from(0.34246666666666666666e-1_f64) * t2457 + F::new(0.5137e-1) * t2468;
    let t2533 = t974 * t978;
    (t2524, t2526, t2527, t2530, t2533)
}

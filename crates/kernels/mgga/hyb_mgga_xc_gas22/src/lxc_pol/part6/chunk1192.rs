//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1192/1455 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1192<F: Float>(t1037: F, t1048: F, t2711: F, t2723: F, t1101: F, t7536: F, t1068: F, t1110: F, t21846: F, t2639: F, t7237: F, t2643: F, t7249: F) -> (F, F, F, F, F) {
    let t22076 = F::new(0.4274e0) * t1037 * t2711 * t2723 * t1048;
    let t22080 = t7536 * t1101;
    let t22084 = t7536 * t1068;
    let t22089 = F::cast_from(0.6233709278045326953e3_f64) * t1110 * t7237 * t21846 * t2639;
    let t22090 = t2643 * t7249;
    (t22076, t22080, t22084, t22089, t22090)
}

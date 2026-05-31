//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 951/1455 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk951<F: Float>(t8651: F, t6530: F, t6533: F, t6592: F, t8648: F, t8676: F, t789: F, t796: F, t238: F, t3344: F, t801: F, t3348: F) -> (F, F, F, F, F, F, F) {
    let t8681 = F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t8651;
    let t8682 = -t6592 + F::cast_from(8.0_f64) / F::cast_from(9.0_f64) * t6530 - t6533 / F::cast_from(3.0_f64) + F::cast_from(4.0_f64) / F::cast_from(9.0_f64) * t8676 - t8681 + t8648;
    let t8683 = t789 * t8682;
    let t8685 = t796 * t8682;
    let t8688 = t238 * t801 * t3344;
    let t8689 = F::cast_from(0.33114e0_f64) * t8688;
    let t8691 = t238 * t801 * t3348;
    (t8681, t8682, t8683, t8685, t8688, t8689, t8691)
}

//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 686/1451 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk686<F: Float>(t601: F, t6642: F, t103: F, t193: F, t197: F, t652: F, t102: F, t133: F, t115: F, t2139: F, t2048: F, t592: F) -> (F, F, F, F, F) {
    let t6644 = F::cast_from(0.58482233974552040708e0_f64) * t601 * t6642;
    let t6653 = F::new(15400.0) / F::new(243.0) * t193 * t652 * t103 * t197;
    let t6654 = t133 * t102;
    let t6680 = t2139 * t115;
    let t6695 = t2048 * t592;
    (t6644, t6653, t6654, t6680, t6695)
}

//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 643/1455 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk643<F: Float>(t616: F, t82: F, t79: F, t1211: F, t3068: F, t3073: F, t3086: F, t3087: F, t3093: F, t623: F, t627: F, t74: F, t81: F) -> (F, F, F) {
    let t3096 = t616 * t82;
    let t3099 = t79 * t616;
    let t3105 = -F::cast_from(2.0_f64) * t3086 * t3087 + t623 * t3068 * t81 / F::cast_from(2.0_f64) + t3093 * t3087 / F::cast_from(4.0_f64) - F::cast_from(4.0_f64) * t3096 * t1211 - t3099 * t3073 - F::cast_from(4.0_f64) * t627 * t3068 - t74 * t3068 * t81;
    (t3096, t3099, t3105)
}

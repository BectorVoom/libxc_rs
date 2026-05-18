//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 644/1455 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk644<F: Float>(t1211: F, t1223: F, t1959: F, t3068: F, t3072: F, t3073: F, t3076: F, t3105: F, t616: F, t618: F, t632: F, t72: F, t85: F) -> F {
    let t3108 = -t3072 * t3073 / F::new(2.0) + F::new(2.0) * t1959 * t3076 - t618 * t3068 + F::new(2.0) * t3068 * t85 + F::new(2.0) * t1211 * t632 + F::new(2.0) * t616 * t1223 + F::new(2.0) * t72 * t3105;
    t3108
}

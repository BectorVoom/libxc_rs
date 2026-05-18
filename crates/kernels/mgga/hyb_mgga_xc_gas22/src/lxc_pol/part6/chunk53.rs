//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 53/1455 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk53<F: Float>(t109: F, t120: F, t116: F, t119: F, t101: F, t105: F, t94: F, t96: F) -> (F, F, F, F) {
    let t121 = t109 * t120;
    let t122 = t116 + t119;
    let t123 = F::new(1.0) / t122;
    let t125 = t94 + F::new(0.3840616724010807e-2) * t96 * t101 * t105 + t121 * t123;
    (t121, t122, t123, t125)
}

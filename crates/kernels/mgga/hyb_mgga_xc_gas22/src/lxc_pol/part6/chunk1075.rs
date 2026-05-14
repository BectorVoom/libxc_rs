//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1075/1345 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1075<F: Float>(t10850: F, t11217: F, t493: F, t7244: F, t7251: F, t7257: F, t7258: F, t7263: F, t7267: F, t7271: F, t9319: F, t9323: F, t9325: F, t9329: F, t9330: F, t9334: F, t9336: F) -> (F,) {
    let t11224 = -t10850 - t7244 - t7251 + t7257 + 0.11696447245269292414e1 * t7258 + 0.19751673498613801407e-1 * t11217 * t493 + 0.48830526149350786811e-3 * t9319 - t9323 + 0.21687162600603479684e-1 * t9325 + t7263 + t7267 + t7271 + t9329 - 24.0 * t9330 + 40.0 * t9334 - t9336;
    (t11224,)
}

//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 913/1345 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk913<F: Float>(t3138: F, t3140: F, t3142: F, t3150: F, t684: F, t687: F, t8476: F, t8479: F, t8481: F, t8485: F, t8491: F, t8493: F, t8501: F, t8502: F, t8506: F, t8511: F, t8513: F, t8514: F, t8519: F, t8521: F, t8526: F) -> (F,) {
    let t8530 = -t8476 - 7.0 / 96.0 * t8479 - t684 * t687 * t8481 / 64.0 + t684 * t3150 * t8485 / 16.0 - t8491 - t684 * t687 * t8493 / 32.0 - t8501 - t3138 * t8502 * t3142 / 24.0 - t3138 * t3140 * t8506 / 48.0 - 7.0 / 144.0 * t8511 * t8513 * t8514 + t3138 * t8519 * t8521 / 12.0 + t8526 * t3140 * t8514 / 16.0;
    (t8530,)
}

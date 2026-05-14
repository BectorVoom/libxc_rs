//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1049/1345 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1049<F: Float>(t10557: F, t10559: F, t10561: F, t10563: F, t10565: F, t10619: F, t10621: F, t10635: F, t10637: F, t10640: F, t10643: F, t3399: F, t3419: F, t4181: F, t6716: F, t3385: F, t3389: F) -> (F, F) {
    let t10776 = 0.11696447245269292414e1 * t3399 * t3419 - 0.11696447245269292414e1 * t6716 * t4181 - t10557 - t10559 - t10561 + t10563 - t10565 - t10619 - t10621 + t10635 - t10637 - t10640 + t10643;
    let t10778 = t3389 * t3385;
    (t10776, t10778)
}

//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1147/1345 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1147<F: Float>(t13: F, t20241: F, t2969: F, t3138: F, t3142: F, t763: F, t8497: F, t8498: F, t8506: F, t191: F, t240: F, t6452: F, t8511: F, t8514: F, t2026: F, t6610: F) -> (F, F, F, F, F, F) {
    let t23872 = t20241 * t13 * t2969;
    let t23883 = t3138 * t8497 * t763 * t3142;
    let t23886 = t3138 * t8498 * t8506;
    let t23889 = t240 * t6452 * t191;
    let t23891 = t8511 * t23889 * t8514;
    let t23894 = t6610 * t2026 * t191;
    (t23872, t23883, t23886, t23889, t23891, t23894)
}

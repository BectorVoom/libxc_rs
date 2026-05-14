//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1258/1345 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1258<F: Float>(t10551: F, t260: F, t10671: F, t10679: F, t2307: F, t2326: F, t2330: F, t29392: F, t29394: F, t29396: F, t29398: F, t29400: F, t29402: F, t29404: F, t3430: F, t4207: F, t6759: F, t856: F, t858: F, t8941: F) -> (F,) {
    let t29494 = t260 * t10551;
    let t29508 = -0.11696447245269292414e1 * t29494 * t858 + 0.11696447245269292414e1 * t10679 * t2326 - 0.5848223622634646207e0 * t10679 * t2330 + 0.11696447245269292414e1 * t6759 * t4207 + 0.11696447245269292414e1 * t856 * t10671 * t2307 - 0.70178683471615754484e1 * t3430 * t8941 - t29392 + t29394 - t29396 - t29398 + t29400 - t29402 + t29404;
    (t29508,)
}

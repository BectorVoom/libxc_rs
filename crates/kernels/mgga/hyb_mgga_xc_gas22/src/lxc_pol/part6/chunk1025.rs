//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1025/1345 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1025<F: Float>(t10325: F, t699: F, t702: F, t10286: F, t10290: F, t10295: F, t10299: F, t10302: F, t10306: F, t10310: F, t10313: F, t10317: F, t10322: F, t3023: F, t572: F, t6278: F, t6279: F, t8288: F, t8291: F, t8293: F, t8294: F) -> (F, F) {
    let t10327 = t699 * t702 * t10325;
    let t10330 = -t6278 - 2.0 / 243.0 * t6279 - 4.0 / 243.0 * t8288 + t8291 - t8293 - 2.0 / 81.0 * t8294 + t10286 / 243.0 - 5.0 / 243.0 * t572 * t10290 + 2.0 / 27.0 * t572 * t10295 + 4.0 / 81.0 * t3023 * t10299 - t10302 / 81.0 - t572 * t10306 / 9.0 - 4.0 / 27.0 * t3023 * t10310 + t10313 / 162.0 - t572 * t10317 / 81.0 + t572 * t10322 / 27.0 - t572 * t10327 / 54.0;
    (t10327, t10330)
}

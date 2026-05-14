//! MGGA_C_RMGGAC lxc pol — lxc_pol part 12 (v4rho3sigma_3) CSE chunk 797/951 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part12_v4rho3sigma_3_chunk797<F: Float>(t35106: F, t35110: F, t35114: F, t35118: F, t39445: F, t39449: F, t39452: F, t39453: F, t39455: F, t39457: F, t39461: F, t39463: F, t39465: F, t39470: F, t39474: F, t39482: F, t39486: F, t8817: F, t931: F) -> (F,) {
    let t39488 = -0.31923449919973379548e-4 * t39445 - 0.1064114997332445985e-4 * t39449 + t39452 + 0.25538759935978703638e-4 * t39453 - 0.53205749866622299248e-5 * t39455 - 0.1064114997332445985e-4 * t39457 - 0.25538759935978703638e-4 * t39461 + 0.25538759935978703638e-4 * t39463 + 0.31923449919973379548e-4 * t39465 - 0.1064114997332445985e-4 * t39470 + 0.42564599893297839398e-5 * t39474 - 0.2363e1 * t931 * t8817 - 0.15243824895787514157e-3 * t35106 + 0.21684485328539747656e-4 * t35110 - 0.30487649791575028314e-3 * t35114 + 0.43368970657079495312e-4 * t35118 - 0.27274661654245341728e-1 * t39482 - 0.13637330827122670864e-1 * t39486;
    (t39488,)
}

//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3328/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3328<F: Float>(t18392: F, t262: F, t11084: F, t18860: F, t2430: F, t4541: F, t51780: F, t5966: F, t5970: F, t62275: F, t62277: F, t62279: F, t62283: F, t62285: F, t62286: F, t62290: F, t62293: F, t62296: F, t775: F) -> F {
    let t63146 = t262 * t18392;
    let t63158 = -F::cast_from(6.0_f64) * t11084 * t4541 * t5966 + F::cast_from(6.0_f64) * t18860 * t2430 * t4541 + F::cast_from(12.0_f64) * t4541 * t63146 * t775 + F::cast_from(12.0_f64) * t51780 * t5970 + t62275 + t62277 + t62279 + t62283 + t62285 + t62286 + t62290 + t62293 + t62296;
    t63158
}

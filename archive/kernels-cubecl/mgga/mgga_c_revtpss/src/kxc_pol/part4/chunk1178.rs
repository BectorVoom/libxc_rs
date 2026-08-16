//! MGGA_C_REVTPSS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 1178/1428 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_kxc_pol_part4_v3rho3_1_chunk1178<F: Float>(t10566: F, t10568: F, t14333: F, t14335: F, t14337: F, t14340: F, t14343: F, t14345: F, t14352: F, t14364: F, t14372: F, t14373: F, t14374: F, t14379: F, t14380: F, t9394: F) -> F {
    let t14610 = t14333 - t14335 - t14337 + t14340 + t14343 + t14345 + t14352 + t9394 + t14364 + t14372 + t14373 + t14374 + t10566 - t10568 + t14379 - t14380;
    t14610
}

//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1099/1243 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk1099<F: Float>(t2140: F, t334: F, t36951: F, t209: F, t7581: F, t9220: F, t7589: F, t36958: F, t73: F, t9249: F, t37000: F, t7579: F, t20: F, t3110: F, t688: F, t7592: F) -> (F, F, F, F, F, F, F, F) {
    let t92223 = t36951 * t334 * t2140;
    let t92226 = t209 * t7581 * t9220;
    let t92227 = t7589 * t92226;
    let t92232 = t209 * t73 * t36958 * t9249;
    let t92233 = t37000 * t7579 * t92232;
    let t92235 = t3110 * t20;
    let t92236 = t688 * t92235;
    let t92237 = t92236 * t7592;
    (t92223, t92226, t92227, t92232, t92233, t92235, t92236, t92237)
}

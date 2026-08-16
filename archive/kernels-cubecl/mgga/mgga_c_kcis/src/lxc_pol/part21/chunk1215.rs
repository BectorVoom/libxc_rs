//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1215/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk1215<F: Float>(t20: F, t3110: F, t688: F, t7592: F, t7583: F, t2381: F, t26579: F, t209: F, t2415: F, t705: F, t73: F, t9251: F) -> (F, F, F, F, F, F) {
    let t92235 = t3110 * t20;
    let t92236 = t688 * t92235;
    let t92237 = t92236 * t7592;
    let t92239 = t92236 * t7583;
    let t92241 = t2381 * t26579;
    let t92242 = t92241 * t7592;
    let t92247 = t209 * t73 * t9251 * t705 * t2415;
    (t92235, t92237, t92239, t92241, t92242, t92247)
}

//! MGGA_C_R2SCAN lxc pol — lxc_pol part 8 (v4rho4_3) CSE chunk 1304/1467 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part8_v4rho4_3_chunk1304<F: Float>(t1561: F, t3229: F, t792: F, t498: F, t9560: F, t3060: F, t5086: F, t2847: F, t7217: F, t288: F, t8629: F, t3162: F, t6887: F, t879: F, t9005: F, t2271: F, t9063: F) -> (F, F, F, F, F, F, F, F) {
    let t31237 = t1561 * t3229 * t792;
    let t31268 = t498 * t9560;
    let t31281 = t5086 * t3060 * t792;
    let t31317 = t7217 * t2847;
    let t31336 = t288 * t8629;
    let t31357 = t6887 * t3162;
    let t31365 = t879 * t9005;
    let t31378 = t2271 * t9063;
    (t31237, t31268, t31281, t31317, t31336, t31357, t31365, t31378)
}

//! MGGA_C_RMGGAC lxc pol — lxc_pol part 12 (v4rho3sigma_3) CSE chunk 746/951 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part12_v4rho3sigma_3_chunk746<F: Float>(t2160: F, t638: F, t8858: F, t8862: F, t2347: F, t839: F, t262: F, t36629: F, t352: F, t8712: F, t7192: F, t16043: F, t9190: F, t9194: F, t9198: F, t2286: F, t35277: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t38559 = t638 * t2160 * t8858;
    let t38560 = 0.81300399444200075504e-3 * t38559;
    let t38562 = t638 * t2160 * t8862;
    let t38563 = 0.81300399444200075504e-3 * t38562;
    let t38564 = t2347 * t839;
    let t38565 = t262 * t38564;
    let t38566 = t36629 * t38565;
    let t38568 = t8712 * t352;
    let t38569 = t262 * t38568;
    let t38570 = t7192 * t38569;
    let t38572 = t16043 * t9190;
    let t38574 = t16043 * t9194;
    let t38576 = t16043 * t9198;
    let t38578 = t35277 * t2286;
    (t38560, t38563, t38564, t38565, t38566, t38568, t38569, t38570, t38572, t38574, t38576, t38578)
}

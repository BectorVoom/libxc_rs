//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 657/988 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk657<F: Float>(t1356: F, t9531: F, t2474: F, t290: F, t289: F, t2448: F, t504: F, t2479: F, t275: F, t2231: F, t534: F, t72: F, t530: F, t8188: F, t302: F, t8328: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t9637 = t1356 * t9531;
    let t9639 = t290 * t2474;
    let t9640 = t289 * t9639;
    let t9642 = t504 * t2448;
    let t9650 = t275 * t2479;
    let t9658 = t534 * t2231;
    let t9659 = t72 * t9658;
    let t9675 = t530 * t8188;
    let t9677 = t302 * t2474;
    let t9678 = t72 * t9677;
    let t10244 = 0.3842256877732895568e-2 * t8328;
    (t9637, t9639, t9640, t9642, t9650, t9658, t9659, t9675, t9677, t9678, t10244)
}

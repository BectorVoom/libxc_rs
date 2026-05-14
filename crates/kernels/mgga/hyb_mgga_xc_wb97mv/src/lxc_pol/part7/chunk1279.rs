//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 1279/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk1279<F: Float>(t26385: F, t26389: F, t26392: F, t26411: F, t26414: F, t26417: F, t30886: F, t30902: F, t30904: F, t30907: F, t30910: F, t30914: F, t22475: F, t22478: F, t22481: F, t22498: F, t22501: F, t22588: F, t22601: F, t30918: F, t30921: F, t30925: F, t30929: F, t30933: F) -> (F, F) {
    let t31359 = 0.16504875e0 * t30886 - 0.33114e0 * t26385 - 0.66228e0 * t26389 - 0.33114e0 * t26392 + 0.11038e1 * t26411 + 0.11038e1 * t26414 - 0.14717333333333333333e1 * t26417 + 0.258925e1 * t30902 + 0.16504875e0 * t30904 + 0.27595e0 * t30907 - 0.66228e0 * t30910 + 0.49671e0 * t30914;
    let t31370 = -0.33114e0 * t30918 - 0.33114e0 * t30921 + 0.248355e0 * t30925 + 0.49671e0 * t30929 + 0.248355e0 * t30933 - 0.18786444444444444445e1 * t22498 + 0.40256666666666666667e0 * t22501 + t22601 + t22588 + 0.27595e0 * t22478 - 0.14717333333333333333e1 * t22475 + 0.27595e0 * t22481;
    (t31359, t31370)
}

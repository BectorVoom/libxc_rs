//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 683/1345 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk683<F: Float>(t1422: F, t986: F, t2457: F, t2505: F, t2545: F, t2550: F, t3461: F, t3472: F, t3486: F, t3491: F, t3497: F, t3499: F, t3503: F, t3507: F, t3511: F) -> (F, F) {
    let t3532 = t1422 * t986;
    let t3546 = -0.17648625e1 * t3486 + 0.3529725e1 * t3491 + t2545 - 0.516475e0 * t2457 - 0.516475e0 * t3461 + 0.1549425e1 * t3472 + 0.31558125e0 * t3497 + 0.6311625e0 * t3499 + t2550 - 0.20839e0 * t2505 - 0.20839e0 * t3503 + 0.312585e0 * t3507 + 0.312585e0 * t3511;
    (t3532, t3546)
}

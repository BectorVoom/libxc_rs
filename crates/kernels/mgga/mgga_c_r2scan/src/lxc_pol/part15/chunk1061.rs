//! MGGA_C_R2SCAN lxc pol — lxc_pol part 15 (v4rho3sigma_5) CSE chunk 1061/1115 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part15_v4rho3sigma_5_chunk1061<F: Float>(t37393: F, t37398: F, t37401: F, t37407: F, t37413: F, t37415: F, t39326: F, t39330: F, t39334: F, t39338: F, t39342: F, t39344: F, t39347: F, t39351: F, t40271: F, t3578: F, t494: F, t97: F) -> (F, F) {
    let t40272 = -t39326 + t39330 + t39334 - t39338 - t39342 + t39344 + t39347 - 0.86737941314158990624e-4 * t37393 - t37398 + 0.92232789896410962678e-3 * t37401 - t39351 + t37407 + t37413 - t37415 - t40271;
    let t40276 = t97 * t3578 * t494;
    (t40272, t40276)
}

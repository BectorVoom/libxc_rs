//! MGGA_C_KCISK lxc pol — lxc_pol part 26 (v4rho3sigma_6) CSE chunk 391/1407 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part26_v4rho3sigma_6_chunk391<F: Float>(t1471: F, t1472: F, t2059: F, t2209: F, t416: F, t140: F, t1469: F, t1470: F, t2221: F, t2225: F, t2242: F, t460: F, t476: F, t479: F, t467: F) -> (F, F, F, F) {
    let t2250 = t1471 * t1472 * t2059;
    let t2253 = t416 * t2209;
    let t2257 = 0.619125e-2 * t2242 * t460 + 0.9286875e-2 * t476 * t2221 - 0.619125e-2 * t476 * t2225 - t1469 - 0.26531111111111111111e-1 * t1470 * t2250 - 0.39796666666666666666e-1 * t140 * t479 * t2253;
    let t2258 = t2257 * t467;
    (t2250, t2253, t2257, t2258)
}

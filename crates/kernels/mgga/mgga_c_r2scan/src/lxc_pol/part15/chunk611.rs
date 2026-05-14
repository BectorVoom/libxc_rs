//! MGGA_C_R2SCAN lxc pol — lxc_pol part 15 (v4rho3sigma_5) CSE chunk 611/1115 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part15_v4rho3sigma_5_chunk611<F: Float>(t3434: F, t3437: F, t3439: F, t1102: F, t1104: F, t3314: F, t261: F, t869: F) -> (F, F, F) {
    let t3441 = t3434 * t3437 * t3439;
    let t3442 = 0.21684485328539747656e-4 * t3441;
    let t3444 = t1102 * t3314 * t1104;
    let t3445 = 0.40650199722100037752e-3 * t3444;
    let t3446 = t869 * t261;
    (t3442, t3445, t3446)
}

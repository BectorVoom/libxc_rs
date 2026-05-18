//! MGGA_C_R2SCAN lxc pol — lxc_pol part 18 (v4rho3sigma_8) CSE chunk 1044/1264 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part18_v4rho3sigma_8_chunk1044<F: Float>(t818: F, t9638: F, t10533: F, t856: F, t352: F, t9769: F, t910: F, t986: F, t113: F, t5086: F, t104: F, t494: F) -> (F, F, F, F, F, F) {
    let t31764 = t9638 * t818;
    let t35213 = t10533 * t856;
    let t35220 = t352 * t9769;
    let t35373 = t986 * t910;
    let t36967 = t113 * t5086;
    let t36985 = t104 * t494;
    (t31764, t35213, t35220, t35373, t36967, t36985)
}

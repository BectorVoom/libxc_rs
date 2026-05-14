//! MGGA_C_R2SCAN lxc pol — lxc_pol part 18 (v4rho3sigma_8) CSE chunk 790/1112 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part18_v4rho3sigma_8_chunk790<F: Float>(t3128: F, t424: F, t4845: F, t6026: F, t7036: F, t7095: F, t7097: F, t765: F, t8642: F, t8643: F, t8644: F, t8646: F, t8647: F, t166: F, t8590: F, t3034: F, t607: F) -> (F, F, F, F) {
    let t9056 = t424 * t3128;
    let t9059 = -t8642 - t8643 + t7036 + t4845 + t8644 + 0.675260332e-1 * t765 * t9056 - t6026 + t7095 + t7097 - t8646 + t8647;
    let t9063 = t8590 * t166;
    let t9066 = t3034 * t607;
    (t9056, t9059, t9063, t9066)
}

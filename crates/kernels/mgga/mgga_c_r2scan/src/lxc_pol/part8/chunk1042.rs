//! MGGA_C_R2SCAN lxc pol — lxc_pol part 8 (v4rho4_3) CSE chunk 1042/1467 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part8_v4rho4_3_chunk1042<F: Float>(t166: F, t9904: F, t3034: F, t955: F, t4873: F, t5039: F, t6026: F, t6047: F, t765: F, t7898: F, t9916: F, t9917: F, t9918: F, t9919: F, t9920: F, t9921: F, t9922: F, t9923: F, t9925: F) -> (F, F, F) {
    let t10297 = t9904 * t166;
    let t10300 = t3034 * t955;
    let t10303 = -t6026 - t9916 - t9917 - t9918 - 0.1714584e0 * t7898 + t9919 + t9920 - t9921 - t4873 + t6047 + t9922 + t9923 - t5039 - t9925 + 0.675260332e-1 * t765 * t10297 + 0.2025780996e0 * t765 * t10300;
    (t10297, t10300, t10303)
}

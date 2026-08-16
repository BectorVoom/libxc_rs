//! MGGA_C_REVTPSS lxc pol — lxc_pol part 29 (v4rho3sigma_4) CSE chunk 1892/2049 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1892<F: Float>(t25950: F, t26271: F, t10073: F, t25920: F, t26260: F, t25898: F, t7527: F, t94849: F, t94383: F, t96221: F, t213: F, t26333: F) -> (F, F, F, F, F) {
    let t96500 = t25950 * t26271;
    let t96503 = t10073 * t25920 * t26260;
    let t96506 = t94849 * t25898 * t7527;
    let t96510 = t94383 * t96221;
    let t96512 = t213 * t26333;
    (t96500, t96503, t96506, t96510, t96512)
}

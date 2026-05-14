//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 1172/1222 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk1172<F: Float>(t15191: F, t50994: F, t1113: F, t13781: F, t3306: F, t3972: F, t824: F, t13808: F, t15151: F, t12182: F, t13792: F, t11378: F, t53566: F, t14733: F, t9917: F, t9923: F) -> (F, F, F, F, F, F, F) {
    let t57643 = t50994 * t15191;
    let t57648 = t3972 * t13781 * t1113 * t824 * t3306;
    let t57652 = t13808 * t15151;
    let t57654 = t13792 * t12182;
    let t57657 = t53566 * t11378;
    let t57661 = t14733 * t9917;
    let t57663 = t14733 * t9923;
    (t57643, t57648, t57652, t57654, t57657, t57661, t57663)
}

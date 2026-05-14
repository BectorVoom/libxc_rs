//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 1120/1222 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk1120<F: Float>(t13808: F, t15200: F, t11749: F, t13917: F, t53447: F, t14125: F, t3781: F, t833: F, t850: F, t1076: F, t1123: F, t837: F, t14677: F, t2503: F, t1113: F, t2118: F, t3028: F, t3972: F, t3975: F) -> (F, F, F, F, F, F) {
    let t56126 = t13808 * t15200;
    let t56129 = t13917 * t53447 * t11749;
    let t56142 = t850 * t3781 * t14125 * t833;
    let t56147 = t850 * t1123 * t1076 * t837 * t833;
    let t56153 = t14677 * t2503;
    let t56166 = t3972 * t3975 * t1113 * t2118 * t3028;
    (t56126, t56129, t56142, t56147, t56153, t56166)
}

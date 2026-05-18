//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 15 (v4rho3sigma_3) CSE chunk 1119/1352 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part15_v4rho3sigma_3_chunk1119<F: Float>(t14054: F, t14105: F, t898: F, t338: F, t353: F, t1176: F, t923: F, t931: F, t3985: F, t376: F, t911: F, t2158: F, t3990: F) -> (F, F, F, F, F, F, F, F) {
    let t14106 = t14054 + t14105;
    let t14107 = t898 * t14106;
    let t14109 = t338 * t353 * t14107;
    let t14113 = t1176 * t923 * t931;
    let t14114 = t14113 * t3985;
    let t14115 = F::new(7.0) / F::new(576.0) * t14114;
    let t14116 = t911 * t376;
    let t14118 = t3990 * t14116 * t2158;
    (t14106, t14107, t14109, t14113, t14114, t14115, t14116, t14118)
}

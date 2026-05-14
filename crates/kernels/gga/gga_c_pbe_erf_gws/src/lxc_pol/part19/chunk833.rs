//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 833/1222 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk833<F: Float>(t2030: F, t985: F, t2032: F, t299: F, t3379: F, t169: F, t242: F, t10201: F, t171: F, t3689: F, t700: F, t3373: F, t532: F, t10008: F, t10010: F, t10012: F, t10015: F, t10017: F, t4503: F, t4506: F, t4513: F, t4539: F, t4542: F, t4602: F, t4744: F, t6918: F, t6932: F, t7984: F, t9764: F) -> (F, F, F, F, F, F) {
    let t10222 = t2030 * t985;
    let t10223 = t10222 * t2032;
    let t10229 = t299 * t3379;
    let t10231 = t169 * t10229 * t242;
    let t10233 = t171 * t10201;
    let t10239 = t169 * t3689 * t700;
    let t10245 = t532 * t3373;
    let t10247 = -t9764 - t6918 + t4503 - t4506 - t4513 + t4539 + t4542 + t10008 + t6932 + t10010 - t7984 + t10012 + t10015 + t10017 + t4602 + t4744;
    (t10223, t10231, t10233, t10239, t10245, t10247)
}

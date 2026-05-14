//! MGGA_C_KCISK lxc pol — lxc_pol part 28 (v4rho3sigma_8) CSE chunk 1387/1456 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part28_v4rho3sigma_8_chunk1387<F: Float>(t34045: F, t34154: F, t23768: F, t9648: F, t415: F, t6944: F, t9956: F, t35081: F, t5074: F, t34097: F, t34125: F, t121456: F, t9649: F, t116120: F, t116960: F, t117128: F, t117133: F, t117136: F, t117138: F, t117140: F, t117146: F, t121156: F, t34039: F, t9652: F, t9922: F) -> (F, F, F) {
    let t121933 = t34154 * t34045;
    let t121937 = t9648 * t23768;
    let t121941 = t415 * t6944 * t9956;
    let t121945 = t5074 * t35081;
    let t121947 = t34125 * t34097;
    let t121949 = t9649 * t121456;
    let t121953 = -t117128 - 0.44218518518518518516e-2 * t117133 + 0.26805555555555555557e-2 * t121933 + t117136 + t117138 + t117140 + 0.10185185185185185186e0 * t121156 * t9652 + 0.39314814814814814818e-1 * t121937 * t9652 + t117146 - 0.13265555555555555555e-1 * t121941 + 0.13888888888888888889e-1 * t116960 * t34039 - 0.58958024691358024689e-2 * t121945 - 0.18518518518518518519e-1 * t121947 - 0.40208333333333333333e-2 * t121949 + 0.20833333333333333334e-1 * t116120 * t9922;
    (t121941, t121945, t121953)
}

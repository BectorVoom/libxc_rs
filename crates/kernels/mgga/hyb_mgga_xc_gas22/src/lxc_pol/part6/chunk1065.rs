//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1065/1345 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1065<F: Float>(t11025: F, t7075: F, t4270: F, t967: F, t2478: F, t2523: F, t4269: F, t2521: F, t3513: F, t3517: F, t4243: F, t7150: F, t7148: F, t10888: F, t10890: F, t10893: F, t10898: F, t10913: F, t10915: F, t10922: F, t10924: F, t6969: F, t7021: F, t9008: F, t9235: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t11027 = 0.96491876992155210402e2 * t7075 * t11025;
    let t11028 = t4270 * t967;
    let t11030 = 2.0 * t2478 * t11028;
    let t11031 = t4269 * t2523;
    let t11032 = t11031 * t967;
    let t11034 = 0.16081979498692535067e2 * t2521 * t11032;
    let t11035 = t3517 * t3513;
    let t11037 = 0.32163958997385070134e2 * t2521 * t11035;
    let t11038 = t4243 * t7150;
    let t11039 = t11038 * t967;
    let t11041 = 0.51726012919273400301e3 * t7148 * t11039;
    let t11056 = 0.264729375e1 * t10888 - 0.3529725e1 * t10890 - 0.17648625e1 * t10893 + 0.3529725e1 * t10915 - t7021 + 0.68863333333333333333e0 * t6969 + 0.13772666666666666667e1 * t9008 - t9235 - 0.516475e0 * t10898 + 0.1549425e1 * t10913 - 0.157790625e0 * t10922 + 0.6311625e0 * t10924;
    (t11027, t11028, t11030, t11031, t11032, t11034, t11035, t11037, t11038, t11039, t11041, t11056)
}

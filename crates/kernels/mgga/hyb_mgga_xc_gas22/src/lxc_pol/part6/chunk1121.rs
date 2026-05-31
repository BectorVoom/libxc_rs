//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1121/1455 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1121<F: Float>(t11038: F, t967: F, t7148: F, t10888: F, t10890: F, t10893: F, t10898: F, t10913: F, t10915: F, t10922: F, t10924: F, t6969: F, t7021: F, t9008: F, t9235: F) -> (F, F, F) {
    let t11039 = t11038 * t967;
    let t11041 = F::cast_from(0.51726012919273400301e3_f64) * t7148 * t11039;
    let t11056 = F::cast_from(0.264729375e1_f64) * t10888 - F::cast_from(0.3529725e1_f64) * t10890 - F::cast_from(0.17648625e1_f64) * t10893 + F::cast_from(0.3529725e1_f64) * t10915 - t7021 + F::cast_from(0.68863333333333333333e0_f64) * t6969 + F::cast_from(0.13772666666666666667e1_f64) * t9008 - t9235 - F::cast_from(0.516475e0_f64) * t10898 + F::cast_from(0.1549425e1_f64) * t10913 - F::cast_from(0.157790625e0_f64) * t10922 + F::cast_from(0.6311625e0_f64) * t10924;
    (t11039, t11041, t11056)
}

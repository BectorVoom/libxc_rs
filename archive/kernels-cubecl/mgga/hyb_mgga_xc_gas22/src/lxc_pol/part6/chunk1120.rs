//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1120/1455 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1120<F: Float>(t1410: F, t3513: F, t2478: F, t4273: F, t967: F, t7075: F, t4270: F, t2523: F, t4269: F, t2521: F, t3517: F, t4243: F, t7150: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t11022 = t1410 * t3513;
    let t11024 = F::cast_from(4.0_f64) * t2478 * t11022;
    let t11025 = t4273 * t967;
    let t11027 = F::cast_from(0.96491876992155210402e2_f64) * t7075 * t11025;
    let t11028 = t4270 * t967;
    let t11030 = F::cast_from(2.0_f64) * t2478 * t11028;
    let t11031 = t4269 * t2523;
    let t11032 = t11031 * t967;
    let t11034 = F::cast_from(0.16081979498692535067e2_f64) * t2521 * t11032;
    let t11035 = t3517 * t3513;
    let t11037 = F::cast_from(0.32163958997385070134e2_f64) * t2521 * t11035;
    let t11038 = t4243 * t7150;
    (t11022, t11024, t11025, t11027, t11028, t11030, t11031, t11032, t11034, t11035, t11037, t11038)
}

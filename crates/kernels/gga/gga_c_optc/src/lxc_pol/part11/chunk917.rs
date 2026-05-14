//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 917/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk917<F: Float>(t18065: F, t4458: F, t18058: F, t9104: F, t18012: F, t4289: F, t18023: F, t3146: F, t894: F, t1179: F, t12727: F, t12729: F, t15874: F, t18055: F, t18059: F, t18062: F, t3244: F, t4457: F, t4464: F, t9093: F, t9102: F, t9116: F, t9122: F) -> (F, F, F, F, F, F) {
    let t18066 = t4458 * t18065;
    let t18069 = t18058 * t9104;
    let t18072 = t4289 * t18012;
    let t18075 = t3146 * t18023;
    let t18076 = t894 * t18075;
    let t18080 = -0.19318136643975017455e-1 * t12727 - 0.33587136305576131526e-2 * t12729 + t9093 - 0.13186481011862155443e4 * t4464 * t18055 + 0.34014423178468276541e6 * t9116 * t18059 - 0.34014423178468276541e6 * t9122 * t18062 + 0.26372962023724310886e4 * t4457 * t18066 + 0.56690705297447127569e5 * t9102 * t18069 + 0.15146801702008125515e1 * t3244 * t18072 + 0.25190352229182098644e-1 * t1179 * t18076 + 0.75734008510040627575e0 * t15874;
    (t18066, t18069, t18072, t18075, t18076, t18080)
}

//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 1233/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk1233<F: Float>(t10545: F, t10559: F, t10563: F, t10564: F, t1312: F, t2013: F, t21949: F, t21953: F, t21962: F, t21976: F, t22292: F, t22311: F, t22314: F, t22316: F, t22320: F, t26129: F, t26136: F, t3: F, t30010: F, t30035: F, t3155: F, t3157: F, t674: F, t8519: F, t8528: F, t8530: F, t8531: F, t8536: F, t8548: F) -> (F,) {
    let t30106 = t21949 / 288.0 + t21953 / 216.0 - 5.0 / 432.0 * t21962 + t21976 / 144.0 + t3155 * t8536 * t1312 * t3 * t674 / 6.0 - t3155 * t8519 * t10564 / 24.0 - t3155 * t8519 * t10559 / 12.0 + t8548 * t10545 * t8531 / 8.0 - t3155 * t3157 * t10563 * t2013 / 48.0 - 7.0 / 144.0 * t8528 * t8530 * t30010 + t8548 * t3157 * t30035 / 16.0 + 41.0 / 48.0 * t26129 + t26136 / 24.0 + t22292 / 48.0 - 5.0 / 144.0 * t22311 + t22314 / 96.0 - 5.0 / 144.0 * t22316 + t22320;
    (t30106,)
}

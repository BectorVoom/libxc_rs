//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 866/1302 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk866<F: Float>(t13585: F, t905: F, t13335: F, t904: F, t916: F, t13290: F, t274: F, t2255: F, t9441: F, t13369: F, t12092: F, t12057: F, t12061: F, t13569: F, t13571: F, t13575: F, t13582: F, t13583: F, t2266: F, t2277: F, t2312: F, t902: F, t914: F, t9658: F, t9669: F) -> (F, F, F, F, F, F, F) {
    let t13586 = t905 * t13585;
    let t13590 = t916 * t904 * t13335;
    let t13593 = t274 * t13290;
    let t13595 = t2255 * t9441 * t13593;
    let t13599 = t916 * t904 * t13369;
    let t13602 = F::cast_from(7.0_f64) / F::cast_from(24.0_f64) * t12092;
    let t13603 = -t13569 - t2312 * t13571 / F::cast_from(64.0_f64) - F::cast_from(119.0_f64) / F::cast_from(1152.0_f64) * t9658 - t13575 + F::cast_from(7.0_f64) / F::cast_from(768.0_f64) * t12057 + F::cast_from(119.0_f64) / F::cast_from(2304.0_f64) * t9669 + t13582 + t13583 - F::cast_from(7.0_f64) / F::cast_from(768.0_f64) * t12061 + t902 * t13586 / F::cast_from(1536.0_f64) - t914 * t13590 / F::cast_from(1536.0_f64) + t2277 * t13595 / F::cast_from(768.0_f64) + F::cast_from(3.0_f64) / F::cast_from(512.0_f64) * t2266 * t13599 - t13602;
    (t13586, t13590, t13593, t13595, t13599, t13602, t13603)
}

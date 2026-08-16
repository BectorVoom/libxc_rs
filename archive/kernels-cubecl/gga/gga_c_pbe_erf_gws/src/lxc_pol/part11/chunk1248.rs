//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1248/1302 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk1248<F: Float>(t45546: F, t37938: F, t1109: F, t3835: F, t3128: F, t45487: F, t11668: F, t13491: F, t1076: F, t13290: F, t13385: F, t13534: F, t2118: F, t2253: F, t2255: F, t2277: F, t2312: F, t3258: F, t3763: F, t3772: F, t3781: F, t37814: F, t45568: F, t9441: F, t9499: F, t9637: F) -> (F, F, F, F, F, F) {
    let t49792 = F::cast_from(7.0_f64) / F::cast_from(3.0_f64) * t45546;
    let t49793 = F::cast_from(35.0_f64) / F::cast_from(72.0_f64) * t37938;
    let t49794 = t3835 * t1109;
    let t49800 = t3128 * t45487;
    let t49802 = t11668 * t13491 / F::cast_from(32.0_f64);
    let t49808 = -F::cast_from(119.0_f64) / F::cast_from(288.0_f64) * t37814 - t2312 * t2255 * t3781 * t13385 / F::cast_from(48.0_f64) + t2277 * t2255 * t9441 * t1076 * t13290 / F::cast_from(256.0_f64) - t2253 * t2255 * t13534 * t3763 / F::cast_from(192.0_f64) + t49792 - t49793 - F::cast_from(3.0_f64) / F::cast_from(128.0_f64) * t9637 * t9499 * t2118 * t49794 - F::cast_from(7.0_f64) / F::cast_from(12.0_f64) * t45568 - t49800 - t49802 - t2277 * t2255 * t3258 * t1076 * t3772 / F::cast_from(512.0_f64);
    (t49792, t49793, t49794, t49800, t49802, t49808)
}

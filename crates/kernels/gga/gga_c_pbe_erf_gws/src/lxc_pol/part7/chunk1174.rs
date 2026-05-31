//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1174/1242 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk1174<F: Float>(t20269: F, t2276: F, t932: F, t2315: F, t6369: F, t6627: F, t2323: F, t6387: F, t19505: F, t20919: F, t20921: F, t20926: F, t20932: F, t20934: F, t20941: F, t20945: F, t2113: F, t2255: F, t2300: F, t2312: F, t904: F, t916: F, t929: F, t9465: F) -> F {
    let t20948 = t2276 * t20269 * t932;
    let t20949 = t20948 * t2315;
    let t20951 = t6627 * t6369;
    let t20953 = t2323 * t6387;
    let t20959 = t20919 - t20921 - t2312 * t2255 * t2113 * t9465 / F::cast_from(64.0_f64) - F::cast_from(119.0_f64) / F::cast_from(1152.0_f64) * t20926 + F::cast_from(5.0_f64) / F::cast_from(128.0_f64) * t20932 * t916 * t904 * t20934 + F::cast_from(119.0_f64) / F::cast_from(384.0_f64) * t20941 - F::cast_from(35.0_f64) / F::cast_from(48.0_f64) * t20945 - F::cast_from(119.0_f64) / F::cast_from(144.0_f64) * t20949 + F::cast_from(35.0_f64) / F::cast_from(48.0_f64) * t20951 + F::cast_from(35.0_f64) / F::cast_from(48.0_f64) * t20953 + F::cast_from(5.0_f64) / F::cast_from(256.0_f64) * t929 * t2300 * t904 * t19505;
    t20959
}

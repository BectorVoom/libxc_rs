//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 20 (v4rho3sigma_8) CSE chunk 1063/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part20_v4rho3sigma_8_chunk1063<F: Float>(t1076: F, t810: F, t1123: F, t2255: F, t11464: F, t3235: F, t875: F, t11514: F, t2345: F, t6287: F, t11901: F, t11907: F, t11911: F, t11913: F, t11915: F, t11919: F, t11923: F, t11927: F, t2312: F, t2343: F, t3247: F, t9123: F, t929: F, t9579: F) -> (F, F, F, F) {
    let t11928 = t1076 * t810;
    let t11930 = t2255 * t1123 * t11928;
    let t11934 = t3235 * t11464 * t875;
    let t11938 = t2345 * t11514 * t6287;
    let t11941 = t2343 * t11901 / F::cast_from(192.0_f64) - t11907 + t11911 + t11913 - F::cast_from(5.0_f64) / F::cast_from(128.0_f64) * t929 * t11915 + F::cast_from(5.0_f64) / F::cast_from(384.0_f64) * t929 * t11919 - t11923 + t11927 - t2312 * t11930 / F::cast_from(384.0_f64) + t9123 - t2343 * t11934 / F::cast_from(1536.0_f64) + t9579 - t3247 * t11938 / F::cast_from(128.0_f64);
    (t11930, t11934, t11938, t11941)
}

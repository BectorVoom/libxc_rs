//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 839/1302 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk839<F: Float>(t1123: F, t13269: F, t2255: F, t1133: F, t816: F, t343: F, t3803: F, t3257: F, t13233: F, t13235: F, t13237: F, t13238: F, t13240: F, t13245: F, t13247: F, t13249: F, t13254: F, t13259: F, t13265: F, t2253: F, t2312: F) -> (F, F, F) {
    let t13271 = t2255 * t1123 * t13269;
    let t13274 = t816 * t1133;
    let t13276 = t3803 * t13274 * t343;
    let t13277 = t3257 * t13276;
    let t13280 = -t13233 - t13235 - t13237 + t13238 - t13240 + t13245 + t13247 - t2253 * t13249 / F::cast_from(256.0_f64) - t2253 * t13254 / F::cast_from(256.0_f64) + t2312 * t13259 / F::cast_from(128.0_f64) - t2253 * t13265 / F::cast_from(256.0_f64) - t2253 * t13271 / F::cast_from(256.0_f64) - t2253 * t13277 / F::cast_from(128.0_f64);
    (t13271, t13277, t13280)
}

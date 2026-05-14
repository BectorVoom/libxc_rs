//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 773/1141 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk773<F: Float>(t13233: F, t13235: F, t13237: F, t13238: F, t13240: F, t13245: F, t13247: F, t13249: F, t13254: F, t13259: F, t13265: F, t13271: F, t13277: F, t2253: F, t2312: F, t11459: F, t2170: F, t3814: F) -> (F, F) {
    let t13280 = -t13233 - t13235 - t13237 + t13238 - t13240 + t13245 + t13247 - t2253 * t13249 / 256.0 - t2253 * t13254 / 256.0 + t2312 * t13259 / 128.0 - t2253 * t13265 / 256.0 - t2253 * t13271 / 256.0 - t2253 * t13277 / 128.0;
    let t13282 = t2170 * t11459 * t3814;
    (t13280, t13282)
}

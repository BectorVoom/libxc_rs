//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1287/1302 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk1287<F: Float>(t50237: F, t50247: F, t50253: F, t50275: F, t50279: F, t50281: F, t50290: F, t50291: F, t50299: F, t50309: F, t50310: F, t21640: F, t50311: F, t50327: F, t50329: F, t50335: F, t50349: F, t50353: F, t50362: F, t50363: F, t50368: F, t50371: F) -> (F, F) {
    let t50589 = t50237 - t50247 + t50253 + t50275 - t50279 + t50281 + t50290 + t50291 + t50299 - t50309 + t50310;
    let t50590 = t50311 - t50327 - t50329 - t50335 + t21640 + t50349 + t50353 + t50362 - t50363 + t50368 + t50371;
    (t50589, t50590)
}

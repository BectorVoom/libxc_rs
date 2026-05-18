//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1273/1302 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk1273<F: Float>(t11412: F, t20833: F, t3791: F, t8978: F, t39388: F, t46596: F, t3134: F, t46446: F, t11778: F, t11782: F, t44230: F, t1109: F, t11994: F, t13257: F, t13263: F, t13385: F, t2253: F, t2255: F, t2266: F, t2312: F, t3258: F, t3752: F, t3772: F, t3781: F, t49932: F, t904: F, t916: F) -> (F, F, F, F, F, F, F) {
    let t50309 = t8978 * t20833 * t3791 * t11412 / F::new(4.0);
    let t50310 = F::new(35.0) / F::new(36.0) * t39388;
    let t50311 = F::new(7.0) / F::new(72.0) * t46596;
    let t50327 = t46446 * t3134 / F::new(24.0);
    let t50329 = t11782 * t11778 / F::new(16.0);
    let t50335 = t44230 * t3134 / F::new(24.0);
    let t50336 = F::new(7.0) / F::new(512.0) * t2266 * t916 * t904 * t49932 - t50309 + t50310 + t50311 + t2312 * t2255 * t11994 * t13257 / F::new(48.0) + t2312 * t2255 * t3258 * t13385 * t1109 / F::new(48.0) + t2312 * t2255 * t3258 * t3752 * t3772 / F::new(96.0) - t50327 - t50329 - t2253 * t2255 * t3781 * t13263 / F::new(96.0) - t50335;
    (t50309, t50310, t50311, t50327, t50329, t50335, t50336)
}

//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 864/1302 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk864<F: Float>(t13408: F, t3131: F, t6523: F, t2168: F, t11977: F, t13520: F, t13522: F, t13527: F, t13529: F, t13531: F, t13538: F, t13541: F, t13545: F, t13549: F, t13553: F, t13557: F, t13561: F, t2277: F, t2343: F, t6275: F, t6637: F, t914: F, t929: F) -> (F, F, F) {
    let t13565 = t6523 * t3131 * t13408;
    let t13567 = F::cast_from(3.0_f64) / F::cast_from(16.0_f64) * t2168 * t13565;
    let t13568 = t13520 + F::cast_from(7.0_f64) / F::cast_from(384.0_f64) * t11977 - t13522 + t13527 + t13529 + t2343 * t13531 / F::cast_from(128.0_f64) + t13538 - t2277 * t13541 / F::cast_from(768.0_f64) + t6275 * t13545 / F::cast_from(32.0_f64) + t6637 * t13549 / F::cast_from(256.0_f64) + F::cast_from(5.0_f64) / F::cast_from(256.0_f64) * t929 * t13553 - t914 * t13557 / F::cast_from(1536.0_f64) + t2343 * t13561 / F::cast_from(128.0_f64) - t13567;
    (t13565, t13567, t13568)
}

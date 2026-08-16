//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1239/1302 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk1239<F: Float>(t2147: F, t3116: F, t337: F, t3717: F, t3791: F, t1123: F, t11464: F, t12024: F, t13252: F, t13253: F, t13334: F, t13408: F, t15150: F, t20304: F, t20307: F, t2253: F, t2255: F, t2277: F, t2343: F, t274: F, t28975: F, t3257: F, t343: F, t3703: F, t3803: F, t44283: F, t49022: F, t49588: F, t49594: F, t49607: F, t6366: F, t6579: F, t816: F) -> (F, F) {
    let t49625 = t3116 * t2147 * t337 * t3791 * t3717 / F::cast_from(8.0_f64);
    let t49629 = -t49588 - t2277 * t2255 * t28975 * t274 * t49022 / F::cast_from(256.0_f64) + t49594 + F::cast_from(5.0_f64) / F::cast_from(32.0_f64) * t6579 * t3257 * t3803 * t816 * t3703 - F::cast_from(5.0_f64) / F::cast_from(64.0_f64) * t2343 * t6366 * t11464 * t13408 - t49607 - F::cast_from(5.0_f64) / F::cast_from(64.0_f64) * t6579 * t12024 * t15150 - t2253 * t2255 * t1123 * t274 * t13334 * t343 / F::cast_from(192.0_f64) - t20304 * t44283 * t20307 * t13252 / F::cast_from(16.0_f64) + t49625 - t2253 * t44283 * t13253 / F::cast_from(192.0_f64);
    (t49625, t49629)
}

//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1239/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk1239(t2147: f64, t3116: f64, t337: f64, t3717: f64, t3791: f64, t1123: f64, t11464: f64, t12024: f64, t13252: f64, t13253: f64, t13334: f64, t13408: f64, t15150: f64, t20304: f64, t20307: f64, t2253: f64, t2255: f64, t2277: f64, t2343: f64, t274: f64, t28975: f64, t3257: f64, t343: f64, t3703: f64, t3803: f64, t44283: f64, t49022: f64, t49588: f64, t49594: f64, t49607: f64, t6366: f64, t6579: f64, t816: f64) -> (f64, f64) {
    let t49625 = t3116 * t2147 * t337 * t3791 * t3717 / 8.0_f64;
    let t49629 = -t49588 - t2277 * t2255 * t28975 * t274 * t49022 / 256.0_f64 + t49594 + 5.0_f64 / 32.0_f64 * t6579 * t3257 * t3803 * t816 * t3703 - 5.0_f64 / 64.0_f64 * t2343 * t6366 * t11464 * t13408 - t49607 - 5.0_f64 / 64.0_f64 * t6579 * t12024 * t15150 - t2253 * t2255 * t1123 * t274 * t13334 * t343 / 192.0_f64 - t20304 * t44283 * t20307 * t13252 / 16.0_f64 + t49625 - t2253 * t44283 * t13253 / 192.0_f64;
    (t49625, t49629)
}

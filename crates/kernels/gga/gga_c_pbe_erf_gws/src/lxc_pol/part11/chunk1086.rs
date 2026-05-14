//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1086/1141 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk1086<F: Float>(t11414: F, t37286: F, t45063: F, t3180: F, t45074: F, t45069: F, t11478: F, t2168: F, t3139: F, t3855: F, t2147: F, t3116: F, t337: F, t3717: F, t3791: F, t1123: F, t11464: F, t12024: F, t13252: F, t13253: F, t13334: F, t13408: F, t15150: F, t20304: F, t20307: F, t2253: F, t2255: F, t2277: F, t2343: F, t274: F, t28975: F, t3257: F, t343: F, t3703: F, t3803: F, t44283: F, t49022: F, t6366: F, t6579: F, t816: F) -> (F, F, F, F, F, F, F) {
    let t49581 = t37286 * t11414 / 4.0;
    let t49585 = 7.0 / 24.0 * t45063;
    let t49588 = t45074 * t3180 / 12.0;
    let t49594 = 7.0 / 12.0 * t45069;
    let t49607 = t2168 * t3139 * t11478 * t3855 / 16.0;
    let t49625 = t3116 * t2147 * t337 * t3791 * t3717 / 8.0;
    let t49629 = -t49588 - t2277 * t2255 * t28975 * t274 * t49022 / 256.0 + t49594 + 5.0 / 32.0 * t6579 * t3257 * t3803 * t816 * t3703 - 5.0 / 64.0 * t2343 * t6366 * t11464 * t13408 - t49607 - 5.0 / 64.0 * t6579 * t12024 * t15150 - t2253 * t2255 * t1123 * t274 * t13334 * t343 / 192.0 - t20304 * t44283 * t20307 * t13252 / 16.0 + t49625 - t2253 * t44283 * t13253 / 192.0;
    (t49581, t49585, t49588, t49594, t49607, t49625, t49629)
}

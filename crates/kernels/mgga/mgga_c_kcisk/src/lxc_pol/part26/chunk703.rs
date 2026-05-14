//! MGGA_C_KCISK lxc pol — lxc_pol part 26 (v4rho3sigma_6) CSE chunk 703/1407 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part26_v4rho3sigma_6_chunk703<F: Float>(t4230: F, t8271: F, t499: F, t7906: F, t498: F, t1504: F, t4304: F, t8010: F, t493: F, t381: F, t7831: F, t8234: F, t8236: F, t8238: F, t8242: F, t8245: F, t8249: F, t8253: F, t8257: F, t8261: F, t8263: F, t8265: F, t8269: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t8272 = t4230 * t8271;
    let t8274 = t499 * t7906;
    let t8275 = t498 * t8274;
    let t8276 = t1504 * t8275;
    let t8278 = t4304 * t8010;
    let t8279 = t498 * t8278;
    let t8280 = t493 * t8279;
    let t8282 = t381 * t7831;
    let t8283 = t498 * t8282;
    let t8284 = t493 * t8283;
    let t8286 = t8234 / 16.0 - t8236 / 8.0 + t8238 / 12.0 + t8242 / 8.0 - t8245 / 12.0 - t8249 / 16.0 - t8253 / 72.0 + t8257 / 24.0 - t8261 / 256.0 + t8263 / 128.0 - t8265 / 96.0 - t8269 / 128.0 + t8272 / 96.0 + t8276 / 256.0 - t8280 / 576.0 - t8284 / 192.0;
    (t8272, t8274, t8275, t8276, t8278, t8279, t8280, t8282, t8283, t8284, t8286)
}

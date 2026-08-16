//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 592/1086 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk592<F: Float>(t4230: F, t8271: F, t499: F, t7906: F, t498: F, t1504: F, t4304: F, t8010: F, t493: F, t381: F, t7831: F, t8234: F, t8236: F, t8238: F, t8242: F, t8245: F, t8249: F, t8253: F, t8257: F, t8261: F, t8263: F, t8265: F, t8269: F) -> (F, F, F, F, F, F, F, F) {
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
    let t8286 = t8234 / F::cast_from(16.0_f64) - t8236 / F::cast_from(8.0_f64) + t8238 / F::cast_from(12.0_f64) + t8242 / F::cast_from(8.0_f64) - t8245 / F::cast_from(12.0_f64) - t8249 / F::cast_from(16.0_f64) - t8253 / F::cast_from(72.0_f64) + t8257 / F::cast_from(24.0_f64) - t8261 / F::cast_from(256.0_f64) + t8263 / F::cast_from(128.0_f64) - t8265 / F::cast_from(96.0_f64) - t8269 / F::cast_from(128.0_f64) + t8272 / F::cast_from(96.0_f64) + t8276 / F::cast_from(256.0_f64) - t8280 / F::cast_from(576.0_f64) - t8284 / F::cast_from(192.0_f64);
    (t8272, t8275, t8276, t8279, t8280, t8283, t8284, t8286)
}

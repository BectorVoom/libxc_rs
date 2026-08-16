//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 592/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk592(t4230: f64, t8271: f64, t499: f64, t7906: f64, t498: f64, t1504: f64, t4304: f64, t8010: f64, t493: f64, t381: f64, t7831: f64, t8234: f64, t8236: f64, t8238: f64, t8242: f64, t8245: f64, t8249: f64, t8253: f64, t8257: f64, t8261: f64, t8263: f64, t8265: f64, t8269: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
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
    let t8286 = t8234 / 16.0_f64 - t8236 / 8.0_f64 + t8238 / 12.0_f64 + t8242 / 8.0_f64 - t8245 / 12.0_f64 - t8249 / 16.0_f64 - t8253 / 72.0_f64 + t8257 / 24.0_f64 - t8261 / 256.0_f64 + t8263 / 128.0_f64 - t8265 / 96.0_f64 - t8269 / 128.0_f64 + t8272 / 96.0_f64 + t8276 / 256.0_f64 - t8280 / 576.0_f64 - t8284 / 192.0_f64;
    (t8272, t8275, t8276, t8279, t8280, t8283, t8284, t8286)
}

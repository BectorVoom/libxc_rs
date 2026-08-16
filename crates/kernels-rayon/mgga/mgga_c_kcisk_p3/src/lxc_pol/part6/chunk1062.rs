//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 1062/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk1062(t31350: f64, t31392: f64, t467: f64, t488: f64, t31133: f64, t499: f64, t498: f64, t1504: f64, t2263: f64, t27181: f64, t31261: f64, t31263: f64, t31267: f64, t31269: f64, t31273: f64, t31275: f64, t31279: f64, t31281: f64, t31284: f64, t31288: f64, t31290: f64, t31293: f64, t31297: f64, t31301: f64, t31303: f64, sigma0: f64) -> (f64, f64, f64, f64) {
    let t31393 = t31350 + t31392;
    let t31394 = t31393 * t467;
    let t31395 = t31394 * sigma0;
    let t31396 = t31395 * t488;
    let t31398 = t499 * t31133;
    let t31399 = t498 * t31398;
    let t31400 = t1504 * t31399;
    let t31402 = t27181 * t2263;
    let t31404 = t31261 / 8.0_f64 - t31263 / 24.0_f64 - 3.0_f64 / 8.0_f64 * t31267 - t31269 / 64.0_f64 + 3.0_f64 / 128.0_f64 * t31273 + 3.0_f64 / 8.0_f64 * t31275 - t31279 / 192.0_f64 - 3.0_f64 / 128.0_f64 * t31281 + t31284 / 24.0_f64 + t31288 / 864.0_f64 - t31290 / 192.0_f64 + t31293 / 192.0_f64 - t31297 / 16.0_f64 + t31301 / 24.0_f64 + 3.0_f64 / 256.0_f64 * t31303 + t31396 / 16.0_f64 + t31400 / 256.0_f64 - 3.0_f64 / 16.0_f64 * t31402;
    (t31396, t31400, t31402, t31404)
}

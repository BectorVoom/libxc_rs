//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 936/957 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk936<F: Float>(t31350: F, t31392: F, t467: F, t488: F, t31133: F, t499: F, t498: F, t1504: F, t2263: F, t27181: F, t31261: F, t31263: F, t31267: F, t31269: F, t31273: F, t31275: F, t31279: F, t31281: F, t31284: F, t31288: F, t31290: F, t31293: F, t31297: F, t31301: F, t31303: F, sigma0: F) -> (F, F, F, F) {
    let t31393 = t31350 + t31392;
    let t31394 = t31393 * t467;
    let t31395 = t31394 * sigma0;
    let t31396 = t31395 * t488;
    let t31398 = t499 * t31133;
    let t31399 = t498 * t31398;
    let t31400 = t1504 * t31399;
    let t31402 = t27181 * t2263;
    let t31404 = t31261 / 8.0 - t31263 / 24.0 - 3.0 / 8.0 * t31267 - t31269 / 64.0 + 3.0 / 128.0 * t31273 + 3.0 / 8.0 * t31275 - t31279 / 192.0 - 3.0 / 128.0 * t31281 + t31284 / 24.0 + t31288 / 864.0 - t31290 / 192.0 + t31293 / 192.0 - t31297 / 16.0 + t31301 / 24.0 + 3.0 / 256.0 * t31303 + t31396 / 16.0 + t31400 / 256.0 - 3.0 / 16.0 * t31402;
    (t31396, t31400, t31402, t31404)
}

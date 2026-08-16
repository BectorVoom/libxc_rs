//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1258/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk1258(t1096: f64, t14865: f64, t5099: f64, t92437: f64, t14765: f64, t28029: f64, t95292: f64, t95294: f64, t95296: f64, t95298: f64, t95301: f64, t95303: f64, t95305: f64, t95307: f64, t95309: f64, t95311: f64, t95313: f64, t95315: f64, t95317: f64, t95319: f64, t95322: f64, t95324: f64, t95327: f64) -> (f64, f64, f64, f64) {
    let t95329 = t1096 * t14865;
    let t95331 = t92437 * t5099;
    let t95333 = t28029 * t14765;
    let t95335 = -11.0_f64 / 18.0_f64 * t95292 + t95294 / 144.0_f64 - t95296 / 9.0_f64 - t95298 / 48.0_f64 + t95301 / 6.0_f64 + 2.0_f64 / 9.0_f64 * t95303 + t95305 / 96.0_f64 - t95307 / 9.0_f64 + t95309 / 432.0_f64 - t95311 / 36.0_f64 - t95313 / 12.0_f64 - t95315 / 24.0_f64 + 11.0_f64 / 18.0_f64 * t95317 + t95319 / 16.0_f64 + t95322 / 12.0_f64 - t95324 / 48.0_f64 + t95327 / 3.0_f64 - t95329 / 12.0_f64 + t95331 / 48.0_f64 - t95333 / 32.0_f64;
    (t95329, t95331, t95333, t95335)
}

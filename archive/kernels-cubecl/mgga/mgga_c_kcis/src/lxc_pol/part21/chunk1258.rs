//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1258/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk1258<F: Float>(t1096: F, t14865: F, t5099: F, t92437: F, t14765: F, t28029: F, t95292: F, t95294: F, t95296: F, t95298: F, t95301: F, t95303: F, t95305: F, t95307: F, t95309: F, t95311: F, t95313: F, t95315: F, t95317: F, t95319: F, t95322: F, t95324: F, t95327: F) -> (F, F, F, F) {
    let t95329 = t1096 * t14865;
    let t95331 = t92437 * t5099;
    let t95333 = t28029 * t14765;
    let t95335 = -F::cast_from(11.0_f64) / F::cast_from(18.0_f64) * t95292 + t95294 / F::cast_from(144.0_f64) - t95296 / F::cast_from(9.0_f64) - t95298 / F::cast_from(48.0_f64) + t95301 / F::cast_from(6.0_f64) + F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t95303 + t95305 / F::cast_from(96.0_f64) - t95307 / F::cast_from(9.0_f64) + t95309 / F::cast_from(432.0_f64) - t95311 / F::cast_from(36.0_f64) - t95313 / F::cast_from(12.0_f64) - t95315 / F::cast_from(24.0_f64) + F::cast_from(11.0_f64) / F::cast_from(18.0_f64) * t95317 + t95319 / F::cast_from(16.0_f64) + t95322 / F::cast_from(12.0_f64) - t95324 / F::cast_from(48.0_f64) + t95327 / F::cast_from(3.0_f64) - t95329 / F::cast_from(12.0_f64) + t95331 / F::cast_from(48.0_f64) - t95333 / F::cast_from(32.0_f64);
    (t95329, t95331, t95333, t95335)
}

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
    let t95335 = -F::new(11.0) / F::new(18.0) * t95292 + t95294 / F::new(144.0) - t95296 / F::new(9.0) - t95298 / F::new(48.0) + t95301 / F::new(6.0) + F::new(2.0) / F::new(9.0) * t95303 + t95305 / F::new(96.0) - t95307 / F::new(9.0) + t95309 / F::new(432.0) - t95311 / F::new(36.0) - t95313 / F::new(12.0) - t95315 / F::new(24.0) + F::new(11.0) / F::new(18.0) * t95317 + t95319 / F::new(16.0) + t95322 / F::new(12.0) - t95324 / F::new(48.0) + t95327 / F::new(3.0) - t95329 / F::new(12.0) + t95331 / F::new(48.0) - t95333 / F::new(32.0);
    (t95329, t95331, t95333, t95335)
}

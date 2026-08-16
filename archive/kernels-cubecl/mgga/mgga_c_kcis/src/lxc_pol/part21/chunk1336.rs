//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1336/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk1336<F: Float>(t95361: F, t95364: F, t95366: F, t95368: F, t95370: F, t95372: F, t95374: F, t95377: F, t95379: F, t95382: F, t95384: F, t95389: F, t95392: F, t95394: F, t95396: F, t95398: F, t95400: F, t95402: F, t95404: F, t95406: F, t95408: F, t95410: F, t95412: F, t95414: F, t95417: F, t95419: F, t95421: F, t95423: F, t95425: F, t95427: F, t95429: F) -> (F, F) {
    let t96617 = F::cast_from(0.68347222222222222224e0_f64) * t95361 - F::cast_from(0.9375e-1_f64) * t95364 - F::cast_from(0.89930555555555555557e-2_f64) * t95366 + F::cast_from(0.28777777777777777778e0_f64) * t95368 + F::cast_from(0.28777777777777777778e0_f64) * t95370 - F::cast_from(0.9112962962962962963e0_f64) * t95372 + F::cast_from(0.625e-1_f64) * t95374 - F::cast_from(0.33333333333333333334e0_f64) * t95377 + F::cast_from(0.11111111111111111111e0_f64) * t95379 + F::cast_from(0.53958333333333333334e-1_f64) * t95382 - F::cast_from(0.20833333333333333333e-1_f64) * t95384;
    let t96640 = -F::cast_from(0.9375e-1_f64) * t95389 + F::cast_from(0.12140625e0_f64) * t95392 - F::cast_from(0.33333333333333333334e0_f64) * t95394 + F::cast_from(0.33333333333333333334e0_f64) * t95396 - F::cast_from(0.4046875e-1_f64) * t95398 - F::cast_from(0.5e0_f64) * t95400 - F::cast_from(0.125e0_f64) * t95402 - F::cast_from(0.26979166666666666667e-1_f64) * t95404 + F::cast_from(0.125e0_f64) * t95406 + F::cast_from(0.20234375e-1_f64) * t95408 - F::cast_from(0.68347222222222222224e0_f64) * t95410 - F::cast_from(0.4046875e-1_f64) * t95412 - F::cast_from(1.0_f64) * t95414 - F::cast_from(0.1875e0_f64) * t95417 - F::cast_from(0.125e0_f64) * t95419 + F::cast_from(0.21583333333333333334e0_f64) * t95421 + F::cast_from(0.5e0_f64) * t95423 - F::cast_from(0.20833333333333333333e-1_f64) * t95425 - F::cast_from(0.1875e0_f64) * t95427 - F::cast_from(0.625e-1_f64) * t95429;
    (t96617, t96640)
}

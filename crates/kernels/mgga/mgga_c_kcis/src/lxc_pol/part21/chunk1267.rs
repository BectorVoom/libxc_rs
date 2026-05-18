//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1267/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk1267<F: Float>(t14809: F, t7748: F, t95389: F, t95392: F, t95394: F, t95396: F, t95398: F, t95400: F, t95402: F, t95404: F, t95406: F, t95408: F, t95410: F, t95412: F, t95414: F, t95417: F, t95419: F, t95421: F, t95423: F, t95425: F, t95427: F) -> (F, F) {
    let t95429 = t7748 * t14809;
    let t95431 = -t95389 / F::new(16.0) + F::new(3.0) / F::new(64.0) * t95392 - F::new(2.0) / F::new(9.0) * t95394 + F::new(2.0) / F::new(9.0) * t95396 - t95398 / F::new(64.0) - t95400 / F::new(3.0) - t95402 / F::new(12.0) - t95404 / F::new(96.0) + t95406 / F::new(12.0) + t95408 / F::new(128.0) - F::new(19.0) / F::new(72.0) * t95410 - t95412 / F::new(64.0) - F::new(2.0) / F::new(3.0) * t95414 - t95417 / F::new(8.0) - t95419 / F::new(12.0) + t95421 / F::new(12.0) + t95423 / F::new(3.0) - t95425 / F::new(72.0) - t95427 / F::new(8.0) - t95429 / F::new(24.0);
    (t95429, t95431)
}

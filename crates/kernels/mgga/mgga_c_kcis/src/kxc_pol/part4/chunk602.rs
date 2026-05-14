//! MGGA_C_KCIS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 602/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part4_v3rho3_1_chunk602<F: Float>(t3423: F, t355: F, t377: F, t3217: F, t3219: F, t376: F, t375: F, t1188: F, t1195: F, t1187: F, t3335: F, t3340: F, t3344: F, t3349: F, t3356: F, t3359: F, t3363: F, t3366: F, t3370: F, sigma0: F) -> (F, F, F, F, F, F, F) {
    let t3424 = t3423 * t355;
    let t3425 = t3424 * sigma0;
    let t3426 = t3425 * t377;
    let t3428 = t3217 * t3219;
    let t3429 = t376 * t3428;
    let t3430 = t375 * t3429;
    let t3432 = t1195 * t1188;
    let t3433 = t1187 * t3432;
    let t3435 = t3335 / 3.0 - t3340 / 12.0 + t3344 / 24.0 - t3349 / 576.0 - 19.0 / 144.0 * t3356 + t3359 / 18.0 + 11.0 / 18.0 * t3363 - 2.0 / 9.0 * t3366 - t3370 / 256.0 + t3426 / 16.0 - t3430 / 72.0 - t3433 / 24.0;
    (t3425, t3426, t3429, t3430, t3432, t3433, t3435)
}

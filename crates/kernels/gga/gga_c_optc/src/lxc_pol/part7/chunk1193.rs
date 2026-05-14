//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1193/1272 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1193<F: Float>(t26314: F, t26319: F, t26339: F, t26343: F, t26363: F, t26365: F, t26367: F, t26369: F, t26372: F, t26376: F, t26379: F, t26382: F, t26385: F, t26388: F, t531: F, t8653: F) -> (F, F) {
    let t26391 = -0.88582716049382716048e0 * t26339 - 0.29896666666666666667e0 * t26343 - 0.54771111111111111111e0 * t26363 - 0.18257037037037037037e0 * t26365 + 0.21908444444444444444e0 * t26367 + 0.97370864197530864196e-1 * t26369 + 0.39862222222222222223e0 * t26314 + 0.10954222222222222222e1 * t26372 - 0.85199506172839506175e-1 * t26376 - 0.82156666666666666667e-1 * t26379 - 0.21908444444444444444e0 * t26382 + 0.65725333333333333332e0 * t26385 - 0.10954222222222222222e0 * t26388 + 0.23917333333333333333e1 * t26319;
    let t26394 = t531 * t8653;
    (t26391, t26394)
}

//! GGA_C_GAPLOC lxc pol — lxc_pol part 46 (v4rhosigma3_11) CSE chunk 397/884 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part46_v4rhosigma3_11_chunk397<F: Float>(t3431: F, t836: F, t568: F, t317: F, t3275: F, t3283: F, t3298: F, t3313: F, t3463: F, t3465: F, t3469: F, t3472: F, t3476: F, t3479: F, t3480: F, t3486: F, t3491: F, t3494: F, t3496: F, t3499: F, t3502: F, t3506: F, t797: F, t813: F, t833: F) -> (F, F, F) {
    let t3507 = t836 * t3431;
    let t3508 = t568 * t3507;
    let t3511 = t3463 + 0.35750489951850426669e0 * t3465 * t317 + t3469 - t3472 + t3275 - t3283 - t3476 + t3479 - 0.35750489951850426669e0 * t797 * t3480 - t3486 - t3491 + t3494 - 0.23005755572352449806e1 * t813 * t3496 - t3298 - t3499 + t3313 + t3502 - t3506 + 0.23005755572352449806e1 * t833 * t3508;
    (t3507, t3508, t3511)
}

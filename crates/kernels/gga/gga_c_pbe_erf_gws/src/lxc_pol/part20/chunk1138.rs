//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 20 (v4rho3sigma_8) CSE chunk 1138/1210 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part20_v4rho3sigma_8_chunk1138<F: Float>(t1134: F, t3068: F, t3972: F, t53240: F, t3902: F, t4386: F, t13792: F, t14404: F, t22379: F, t51156: F, t51168: F, t53503: F, t53509: F, t53516: F, t53795: F, t54550: F, t56483: F, t56491: F, t56495: F, t56500: F, t56505: F, t6793: F, t8793: F) -> (F,) {
    let t56511 = t3972 * t53240 * t1134 * t3068;
    let t56513 = t4386 * t3902;
    let t56514 = t13792 * t56513;
    let t56518 = -t56483 / 48.0 + t8793 * t54550 / 24.0 + t22379 * t14404 / 24.0 - t6793 * t56491 / 16.0 - t56495 / 96.0 + t56500 / 192.0 + t56505 / 192.0 - 35.0 / 432.0 * t51156 + 35.0 / 216.0 * t51168 - t53503 + t53509 - t56511 / 768.0 + t53516 - t56514 / 24.0 - t8793 * t53795 / 8.0;
    (t56518,)
}

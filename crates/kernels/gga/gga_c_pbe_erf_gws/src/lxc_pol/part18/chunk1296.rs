//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 18 (v4rho3sigma_6) CSE chunk 1296/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part18_v4rho3sigma_6_chunk1296<F: Float>(t14404: F, t22379: F, t51156: F, t51168: F, t53503: F, t53509: F, t53516: F, t53795: F, t54550: F, t56483: F, t56491: F, t56495: F, t56500: F, t56505: F, t56511: F, t56514: F, t6793: F, t8793: F) -> F {
    let t56518 = -t56483 / F::cast_from(48.0_f64) + t8793 * t54550 / F::cast_from(24.0_f64) + t22379 * t14404 / F::cast_from(24.0_f64) - t6793 * t56491 / F::cast_from(16.0_f64) - t56495 / F::cast_from(96.0_f64) + t56500 / F::cast_from(192.0_f64) + t56505 / F::cast_from(192.0_f64) - F::cast_from(35.0_f64) / F::cast_from(432.0_f64) * t51156 + F::cast_from(35.0_f64) / F::cast_from(216.0_f64) * t51168 - t53503 + t53509 - t56511 / F::cast_from(768.0_f64) + t53516 - t56514 / F::cast_from(24.0_f64) - t8793 * t53795 / F::cast_from(8.0_f64);
    t56518
}

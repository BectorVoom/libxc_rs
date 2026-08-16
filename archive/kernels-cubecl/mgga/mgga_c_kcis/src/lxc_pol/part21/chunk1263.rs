//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1263/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk1263<F: Float>(t95361: F, t95364: F, t95366: F, t95368: F, t95370: F, t95372: F, t95374: F, t95377: F, t95379: F, t95382: F, t95384: F, t8069: F, t92486: F) -> (F, F) {
    let t95386 = F::cast_from(19.0_f64) / F::cast_from(72.0_f64) * t95361 - t95364 / F::cast_from(16.0_f64) - t95366 / F::cast_from(288.0_f64) + t95368 / F::cast_from(9.0_f64) + t95370 / F::cast_from(9.0_f64) - F::cast_from(19.0_f64) / F::cast_from(54.0_f64) * t95372 + t95374 / F::cast_from(24.0_f64) - F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t95377 + F::cast_from(2.0_f64) / F::cast_from(27.0_f64) * t95379 + t95382 / F::cast_from(48.0_f64) - t95384 / F::cast_from(72.0_f64);
    let t95389 = t92486 * t8069;
    (t95386, t95389)
}

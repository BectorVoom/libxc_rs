//! MGGA_C_KCIS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 386/1419 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part5_v3rho3_2_chunk386<F: Float>(t1360: F, t1403: F, t1404: F, t1405: F, t1420: F, t1455: F, t486: F, t510: F, t538: F) -> F {
    let t1457 = -t1403 - F::cast_from(0.23426533963880895498e-2_f64) * t1404 * t1405 - F::cast_from(0.46853067927761790996e-2_f64) * t510 * t1420 - t1360 * t538 - t486 * t1455;
    t1457
}

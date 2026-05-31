//! MGGA_C_KCIS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 179/1419 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part5_v3rho3_2_chunk179<F: Float>(t542: F, t545: F, t486: F, t344: F) -> (F, F, F) {
    let t546 = t542 * t545;
    let t549 = t486 * t486;
    let t551 = F::cast_from(0.98556445e-3_f64) * t344 * t546 - F::cast_from(2.0_f64) * t549;
    let t552 = F::cast_from(1.0_f64) / t551;
    (t546, t551, t552)
}

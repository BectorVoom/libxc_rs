//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 884/1397 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk884<F: Float>(t21078: F, t5701: F, t531: F, t7080: F, t833: F, t12185: F, t12147: F, t7068: F, t1368: F, t1938: F, t5477: F, t16884: F) -> (F, F, F, F) {
    let t21079 = t5701 * t21078;
    let t21082 = t7080 * t531;
    let t21083 = t21082 * t833;
    let t21084 = t12185 * t21083;
    let t21087 = t12147 * t7068;
    let t21088 = t1368 * t21087;
    let t21097 = t5477 * t1938;
    let t21098 = t16884 * t21097;
    (t21079, t21084, t21088, t21098)
}

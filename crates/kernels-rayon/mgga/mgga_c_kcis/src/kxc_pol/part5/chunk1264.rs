//! MGGA_C_KCIS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 1264/1419 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_kxc_pol_part5_v3rho3_2_chunk1264(t21078: f64, t5701: f64, t531: f64, t7080: f64, t833: f64, t12185: f64, t12147: f64, t7068: f64, t1368: f64, t1938: f64, t5477: f64, t16884: f64) -> (f64, f64, f64, f64) {
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

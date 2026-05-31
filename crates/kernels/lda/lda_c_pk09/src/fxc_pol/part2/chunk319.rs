//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 319/1113 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk319<F: Float>(t1435: F, t383: F, t1222: F, t373: F, t332: F, t366: F, t356: F, t396: F, t225: F, t226: F) -> (F, F, F, F, F, F, F) {
    let t1457 = F::cast_from(6.211752672544321_f64) * t383 * t1435;
    let t1458 = t1222 * t373;
    let t1460 = F::cast_from(0.013716887843283197_f64) * t332 * t1458;
    let t1462 = F::cast_from(1.6457779058161184_f64) * t366 * t1435;
    let t1464 = F::cast_from(0.6268457032291772_f64) * t356 * t1435;
    let t1466 = F::cast_from(0.7380249726277691_f64) * t396 * t1435;
    let t1468 = F::cast_from(1.0_f64) / t226 / t225;
    (t1457, t1458, t1460, t1462, t1464, t1466, t1468)
}

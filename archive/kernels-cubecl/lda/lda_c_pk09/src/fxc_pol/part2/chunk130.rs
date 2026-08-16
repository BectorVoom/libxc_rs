//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 130/1113 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk130<F: Float>(t142: F, t409: F, t297: F, t311: F, t332: F, t335: F, t341: F, t343: F, t348: F, t356: F, t360: F, t366: F, t374: F, t383: F, t396: F, t404: F, t408: F) -> (F, F) {
    let t410 = t142 * t409;
    let t413 = t297 * t343 - F::cast_from(22.07984838129906_f64) * t335 - F::cast_from(2.700122570973095_f64) * t341 - F::cast_from(3.7610742193750633_f64) * t348 * t311 + F::cast_from(1.8805371096875316_f64) * t356 * t311 + F::cast_from(19.489173774580152_f64) * t360 * t311 + F::cast_from(4.937333717448355_f64) * t366 * t311 - F::cast_from(0.04115066352984959_f64) * t332 * t374 + F::cast_from(18.635258017632964_f64) * t383 * t311 - F::cast_from(2.2140749178833072_f64) * t396 * t311 - F::cast_from(2.427516195194328_f64) * t404 * t311 - F::cast_from(3.5540878740919255_f64) * t408 * t410;
    (t410, t413)
}

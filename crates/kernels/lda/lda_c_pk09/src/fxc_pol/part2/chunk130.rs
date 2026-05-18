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
    let t413 = t297 * t343 - F::new(22.07984838129906) * t335 - F::new(2.700122570973095) * t341 - F::new(3.7610742193750633) * t348 * t311 + F::new(1.8805371096875316) * t356 * t311 + F::new(19.489173774580152) * t360 * t311 + F::new(4.937333717448355) * t366 * t311 - F::new(0.04115066352984959) * t332 * t374 + F::new(18.635258017632964) * t383 * t311 - F::new(2.2140749178833072) * t396 * t311 - F::new(2.427516195194328) * t404 * t311 - F::new(3.5540878740919255) * t408 * t410;
    (t410, t413)
}

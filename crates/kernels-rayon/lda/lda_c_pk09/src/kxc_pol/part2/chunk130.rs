//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 130/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk130(t142: f64, t409: f64, t297: f64, t311: f64, t332: f64, t335: f64, t341: f64, t343: f64, t348: f64, t356: f64, t360: f64, t366: f64, t374: f64, t383: f64, t396: f64, t404: f64, t408: f64) -> (f64, f64) {
    let t410 = t142 * t409;
    let t413 = t297 * t343 - 22.07984838129906_f64 * t335 - 2.700122570973095_f64 * t341 - 3.7610742193750633_f64 * t348 * t311 + 1.8805371096875316_f64 * t356 * t311 + 19.489173774580152_f64 * t360 * t311 + 4.937333717448355_f64 * t366 * t311 - 0.04115066352984959_f64 * t332 * t374 + 18.635258017632964_f64 * t383 * t311 - 2.2140749178833072_f64 * t396 * t311 - 2.427516195194328_f64 * t404 * t311 - 3.5540878740919255_f64 * t408 * t410;
    (t410, t413)
}

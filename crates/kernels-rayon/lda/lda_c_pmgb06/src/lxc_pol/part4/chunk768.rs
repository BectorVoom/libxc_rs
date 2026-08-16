//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 768/1478 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk768(t477: f64, t760: f64, t332: f64, t5084: f64, t5083: f64, t1601: f64, t851: f64, t1381: f64, t5068: f64, t1531: f64, t465: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t5085 = t760 * t477;
    let t5086 = t5085 * t332;
    let t5087 = t5084 * t5086;
    let t5089 = 2.0_f64 / 27.0_f64 * t5083 * t5087;
    let t5090 = t1601 * t851;
    let t5091 = t5090 * t1381;
    let t5093 = 4.0_f64 / 45.0_f64 * t5068 * t5091;
    let t5094 = t465 * t1531;
    (t5085, t5086, t5087, t5089, t5090, t5091, t5093, t5094)
}

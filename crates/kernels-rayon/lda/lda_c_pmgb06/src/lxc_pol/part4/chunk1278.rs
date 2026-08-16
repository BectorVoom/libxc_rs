//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1278/1478 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1278(t13026: f64, t13031: f64, t15872: f64, t13020: f64, t15880: f64, t5084: f64, t2381: f64, t332: f64, t477: f64, t5083: f64, t5077: f64, t5094: f64) -> (f64, f64, f64, f64) {
    let t16809 = 16.0_f64 / 81.0_f64 * t13026 * t13031 * t15872;
    let t16812 = 8.0_f64 / 27.0_f64 * t13020 * t5084 * t15880;
    let t16814 = t2381 * t477 * t332;
    let t16817 = 2.0_f64 / 27.0_f64 * t5083 * t5084 * t16814;
    let t16820 = 4.0_f64 / 45.0_f64 * t5077 * t5094 * t16814;
    (t16809, t16812, t16817, t16820)
}

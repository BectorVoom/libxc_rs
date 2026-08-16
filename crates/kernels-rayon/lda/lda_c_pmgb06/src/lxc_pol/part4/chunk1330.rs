//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1330/1478 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1330(t13035: f64, t5083: f64, t6642: f64, t2064: f64, t760: f64, t332: f64, t5084: f64, t16821: f64, t13026: f64, t13031: f64, t16825: f64, t13020: f64, t16830: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t17482 = 4.0_f64 / 27.0_f64 * t5083 * t13035 * t6642;
    let t17483 = t760 * t2064;
    let t17484 = t17483 * t332;
    let t17487 = 4.0_f64 / 27.0_f64 * t5083 * t5084 * t17484;
    let t17490 = 2.0_f64 / 27.0_f64 * t5083 * t5084 * t16821;
    let t17493 = 16.0_f64 / 81.0_f64 * t13026 * t13031 * t16825;
    let t17496 = 8.0_f64 / 27.0_f64 * t13020 * t5084 * t16830;
    (t17482, t17483, t17484, t17487, t17490, t17493, t17496)
}
